use super::po::user::{ActiveModel, Column};
use super::po::user::{Entity, Model};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use starriver_domain::user::entity::User;
use starriver_domain::user::repository::UserRepository;
use starriver_domain::user::value_object::{Password, Username};
use starriver_infrastructure::error::error::ApiError;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct UserRepositoryImpl {
    conn: &'static DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        UserRepositoryImpl { conn }
    }
}

impl UserRepository for UserRepositoryImpl {
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
        .map(model_to_entity)
        .map_err(ApiError::from)
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
        .map(model_to_entity)
        .map_err(ApiError::from)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError> {
        Entity::find()
            .filter(Column::Username.eq(username))
            .one(self.conn)
            .await
            .map(|e| e.map(model_to_entity))
            .map_err(ApiError::from)
    }
}

#[inline]
fn model_to_entity(m: Model) -> User {
    let username = Username::new(m.username.as_str()).expect("Username");
    let password = Password::restore_by_hashed_pwd(m.password.as_str(), OffsetDateTime::now_utc())
        .expect("Password");
    User {
        id: m.id,
        username,
        password,
        state: Default::default(),
        created_at: m.create_at,
        login_events: vec![],
    }
}
