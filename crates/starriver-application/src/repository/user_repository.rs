use crate::db::user_do::ActiveModel;
use crate::db::user_do::Model;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use starriver_domain::user::entity::User;
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::value_object::{Password, Username};
use starriver_infrastructure::error::error::ApiError;
use time::OffsetDateTime;

pub struct DefaultUserRepository {
    pub conn: &'static DatabaseConnection,
}

impl UserRepository for DefaultUserRepository {
    async fn insert(&self, user: User) -> Result<User, ApiError> {
        ActiveModel {
            id: Set(user.id),
            username: Set(user.username.as_str().to_string()),
            password: Set(user.password.hashed_password_string().to_string()),
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
            create_at: Set(OffsetDateTime::now_utc()),
            update_at: Set(None),
        }
        .update(self.conn)
        .await
        .map(model_to_entity)?
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
        state: Default::default(),
        created_at: m.create_at,
        login_events: vec![],
    })
}
