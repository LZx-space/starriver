use anyhow::Error;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use uuid::Uuid;

use crate::domain::user::aggregate::User;
use crate::domain::user::repository::UserRepository;
use crate::infrastructure::security::authentication::util::hash_password;

pub use super::po::user::ActiveModel;
pub use super::po::user::Entity;

pub struct UserRepositoryImpl {
    conn: &'static DatabaseConnection,
    password_salt: &'static str,
}

impl UserRepositoryImpl {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        UserRepositoryImpl {
            conn,
            password_salt: "stariver",
        }
    }
}

impl UserRepository for UserRepositoryImpl {
    async fn insert(&self, user: User) -> Option<Error> {
        let model = ActiveModel {
            id: Set(Uuid::now_v7()),
            username: Set(user.username),
            password: Set(user.password),
            create_at: Set(Local::now()),
            update_at: Set(None),
        };
        model
            .insert(self.conn)
            .await
            .map_err(|e| Error::new(e))
            .err()
    }

    async fn update(&self, user: User) -> Option<Error> {
        todo!()
    }

    async fn find_by_username(&self, username: &str) -> Result<User, Error> {
        match hash_password("password", self.password_salt) {
            Ok(hash_string) => Ok(User {
                username: username.to_string(),
                password: hash_string.to_string(),
                phone: "".to_string(),
                email: "".to_string(),
            }),
            Err(err) => Err(Error::msg(err.to_string())),
        }
    }
}
