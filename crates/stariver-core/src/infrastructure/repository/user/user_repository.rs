use std::fmt::Error;

use sea_orm::DatabaseConnection;

use crate::domain::user::aggregate::User;
use crate::domain::user::repository::UserRepository;
use crate::infrastructure::security::authentication::util::hash_password;

pub struct UserRepositoryImpl {
    pub conn: &'static DatabaseConnection,
}

impl UserRepository for UserRepositoryImpl {
    async fn insert(&self, user: User) -> Result<bool, Error> {
        todo!()
    }

    async fn update(&self, user: User) -> Result<bool, Error> {
        todo!()
    }

    async fn find_by_username(&self, username: &str) -> Result<User, Error> {
        match hash_password("password", "ABCDEFGH") {
            Ok(hash_string) => Ok(User {
                username: username.to_string(),
                password: hash_string.to_string(),
                phone: "".to_string(),
                email: "".to_string(),
            }),
            Err(_) => Err(Error::default()),
        }
    }
}
