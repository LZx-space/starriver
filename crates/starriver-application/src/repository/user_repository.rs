use crate::db::user_do::ActiveModel;
use crate::db::user_do::Entity;
use crate::db::user_do::Model;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use starriver_domain::user::entity::User;
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::value_object::{Password, Username};
use starriver_infrastructure::error::error::ApiError;
use starriver_infrastructure::error::error::Cause;
use time::OffsetDateTime;

pub struct DefaultUserRepository {
    pub conn: &'static DatabaseConnection,
}

impl UserRepository for DefaultUserRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError> {
        Entity::find_by_username(username)
            .one(self.conn)
            .await?
            .map(model_to_entity)
            .transpose()
    }

    async fn insert(&self, user: User) -> Result<User, ApiError> {
        let username = user.username.as_str();
        let found = self.find_by_username(username).await?;
        if found.is_some() {
            return Err(ApiError::new(
                Cause::ClientBadRequest,
                "username already exists",
            ));
        }
        ActiveModel {
            id: Set(user.id),
            username: Set(username.to_string()),
            password: Set(user.password.hashed_password_string().to_string()),
            state: Set(crate::db::user_do::UserStateDo::Inactive),
            create_at: Set(OffsetDateTime::now_utc()),
            update_at: Set(None),
        }
        .insert(self.conn)
        .await
        .map(model_to_entity)?
    }

    async fn update(&self, user: User) -> Result<User, ApiError> {
        ActiveModel {
            id: Set(user.id),
            username: Set(user.username.as_str().to_string()),
            password: Set(user.password.hashed_password_string().to_string()),
            state: Set(user.state.into()),
            create_at: NotSet,
            update_at: Set(Some(OffsetDateTime::now_utc())),
        }
        .update(self.conn)
        .await
        .map(model_to_entity)?
    }

    async fn delete(&self, user_id: uuid::Uuid) -> Result<bool, ApiError> {
        let result = Entity::delete_by_id(user_id)
            .exec(self.conn)
            .await?
            .rows_affected
            > 0;
        Ok(result)
    }
}

#[inline]
fn model_to_entity(m: Model) -> Result<User, ApiError> {
    let username = Username::new(m.username.as_str())?;
    let password = Password::restore_by_hashed_pwd(m.password.as_str(), OffsetDateTime::now_utc())?;
    Ok(User {
        id: m.id,
        username,
        password,
        state: m.state.into(),
        created_at: m.create_at,
        login_events: vec![],
    })
}
