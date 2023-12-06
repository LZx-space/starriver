use std::fmt::Error;

use sea_orm::prelude::async_trait::async_trait;

use crate::domain::user::aggregate::User;

#[async_trait]
pub trait UserRepository {
    async fn insert(&self, user: User) -> Result<bool, Error>;

    async fn update(&self, user: User) -> Result<bool, Error>;

    async fn find_by_username(&self, username: &str) -> Result<User, Error>;
}
