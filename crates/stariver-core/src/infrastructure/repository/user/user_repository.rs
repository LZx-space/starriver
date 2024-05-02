use anyhow::Error;
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::domain::user::aggregate::User;
use crate::domain::user::repository::UserRepository;
pub use crate::infrastructure::repository::user::po::user::Column;
pub use crate::infrastructure::security::authentication::util::hash_password;

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
    async fn insert(&self, user: User) -> Result<User, Error> {
        let model = hash_password(&user.password, self.password_salt)
            .map_err(|e| Error::msg(e.to_string()))
            .map(|e| ActiveModel {
                id: Set(Uuid::now_v7()),
                username: Set(user.username),
                password: Set(e.to_string()),
                create_at: Set(Local::now()),
                update_at: Set(None),
            });

        match model {
            Ok(am) => am
                .insert(self.conn)
                .await
                .map(|m| User {
                    id: m.id,
                    username: m.username,
                    password: m.password,
                })
                .map_err(|e| Error::new(e)),
            Err(err) => Err(err),
        }
    }

    async fn update(&self, user: User) -> Option<Error> {
        todo!()
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        Entity::find()
            .filter(Column::Username.eq(username))
            .one(self.conn)
            .await
            .map(|e| {
                e.map(|m| User {
                    id: m.id,
                    username: m.username,
                    password: m.password,
                })
            })
            .map_err(|e| Error::new(e))
    }
}
