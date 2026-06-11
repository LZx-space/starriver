use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveValue::Unchanged;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use starriver_identity_domain::user::entity::User;
use starriver_identity_domain::user::repository::UserRepository;
use starriver_shared_base::error::RepositoryError;
use starriver_shared_base::repository::Revision;
use starriver_shared_framework::error_mapping::db_2_repo_error;
use time::OffsetDateTime;

use crate::port_out::persistence::po::user_po::ActiveModel;
use crate::port_out::persistence::po::user_po::Column;
use crate::port_out::persistence::po::user_po::Entity;

pub struct DefaultUserRepository {
    conn: DatabaseConnection,
}

impl DefaultUserRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl UserRepository for DefaultUserRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError> {
        Entity::find()
            .filter(Column::Username.eq(username))
            .one(&self.conn)
            .await
            .map(|e| {
                e.map(|e| User::from_repo(e.id, e.username, e.password, e.email, e.state.into()))
            })
            .map_err(db_2_repo_error)
    }

    async fn insert(&self, user: User) -> Result<User, RepositoryError> {
        let (id, username, password, email, state) = user.dissolve();
        ActiveModel {
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
        .map_err(db_2_repo_error)
        .map(|m| User::from_repo(m.id, m.username, m.password, m.email, m.state.into()))
    }

    async fn delete(&self, user_id: uuid::Uuid) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(user_id)
            .exec(&self.conn)
            .await
            .map(|e| e.rows_affected > 0)
            .map_err(db_2_repo_error)
    }

    async fn update(&self, user: Revision<User>) -> Result<User, RepositoryError> {
        let (original, modified) = user.dissolve();
        let (user_id, username, password, email, state) = original.dissolve();
        let (_, new_username, new_password, new_email, new_state) = modified.dissolve();
        let mut username = Unchanged(username.as_str().to_string());
        username.set_if_not_equals(new_username.as_str().to_string());
        let mut password = Unchanged(password.as_str().to_string());
        password.set_if_not_equals(new_password.as_str().to_string());
        let mut email = Unchanged(email.to_string());
        email.set_if_not_equals(new_email.to_string());
        let mut state = Unchanged(state.into());
        state.set_if_not_equals(new_state.into());

        ActiveModel {
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
        .map_err(db_2_repo_error)
        .map(|m| User::from_repo(m.id, m.username, m.password, m.email, m.state.into()))
    }
}
