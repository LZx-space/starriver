use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveValue::Unchanged;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use starriver_identity_domain::error::DomainError;
use starriver_identity_domain::user::entity::User;
use starriver_identity_domain::user::repository::UserRepository;
use starriver_identity_domain::user::value_object::Email;
use starriver_identity_domain::user::value_object::Password;
use starriver_identity_domain::user::value_object::UserState;
use starriver_identity_domain::user::value_object::Username;
use starriver_shared_base::error::RepositoryError;
use starriver_shared_base::regex_patterns::Patterns;
use starriver_shared_base::repository::Revision;
use starriver_shared_framework::error_mapping::db_error_2_repo_error;
use time::OffsetDateTime;

use crate::port_out::persistence::po::user_po::ActiveModel;
use crate::port_out::persistence::po::user_po::Column;
use crate::port_out::persistence::po::user_po::Entity;
use crate::port_out::persistence::po::user_po::Model;

pub struct DefaultUserRepository<T> {
    conn: T,
    patterns: Patterns,
}

impl<T> DefaultUserRepository<T> {
    pub fn new(conn: T, patterns: Patterns) -> Self {
        Self { conn, patterns }
    }
}

impl<T> UserRepository for DefaultUserRepository<T>
where
    T: ConnectionTrait,
{
    async fn find_by_username(&self, username: String) -> Result<Option<User>, RepositoryError> {
        Entity::find()
            .filter(Column::Username.eq(username))
            .one(&self.conn)
            .await
            .map_err(db_error_2_repo_error)?
            .map(|e| self.model_to_entity(e))
            .transpose()
            .map_err(|e| RepositoryError::Infrastructure(e.to_string()))
    }

    async fn insert(&self, user: User) -> Result<User, RepositoryError> {
        let (id, username, password, email, state) = user.dissolve();
        let model = ActiveModel {
            id: Set(id),
            username: Set(username.to_string()),
            password: Set(password.as_str().to_string()),
            email: Set(email.to_string()),
            state: Set(state.into()),
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: NotSet,
        }
        .insert(&self.conn)
        .await
        .map_err(db_error_2_repo_error)?;
        self.model_to_entity(model)
            .map_err(|e| RepositoryError::Infrastructure(e.to_string()))
    }

    async fn delete(&self, user_id: uuid::Uuid) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(user_id)
            .exec(&self.conn)
            .await
            .map(|e| e.rows_affected > 0)
            .map_err(db_error_2_repo_error)
    }

    async fn update(&self, user: Revision<User>) -> Result<User, RepositoryError> {
        let (original, modified) = user.dissolve();
        let (user_id, username, password, email, state) = original.dissolve();
        let (_, new_username, new_password, new_email, new_state) = modified.dissolve();
        // 更新用户
        let mut username = Unchanged(username.as_str().to_string());
        username.set_if_not_equals(new_username.as_str().to_string());

        let mut password = Unchanged(password.as_str().to_string());
        password.set_if_not_equals(new_password.as_str().to_string());

        let mut email = Unchanged(email.to_string());
        email.set_if_not_equals(new_email.to_string());

        let mut state = Unchanged(state.into());
        state.set_if_not_equals(new_state.into());

        let model = ActiveModel {
            id: Unchanged(user_id),
            username,
            password,
            email,
            state,
            created_at: NotSet,
            updated_at: Set(Some(OffsetDateTime::now_utc())),
        }
        .update(&self.conn)
        .await
        .map_err(db_error_2_repo_error)?;
        self.model_to_entity(model)
            .map_err(|e| RepositoryError::Infrastructure(e.to_string()))
    }
}

impl<T> DefaultUserRepository<T> {
    #[inline]
    fn model_to_entity(&self, m: Model) -> Result<User, DomainError> {
        let username = Username::new(&m.username, &self.patterns.username)?;
        let hashed_pwd = Password::new(&m.password)?;
        let email = Email::new(&m.email, &self.patterns.email)?;
        let state = UserState::from(m.state);
        Ok(User::new(m.id, username, hashed_pwd, email, state))
    }
}
