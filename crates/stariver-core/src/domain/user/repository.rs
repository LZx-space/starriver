use anyhow::Error;
use sea_orm::prelude::async_trait::async_trait;
use crate::domain::user::aggregate::User;

#[async_trait]
pub trait UserRepository {
    async fn insert(&self, user: User) -> Result<User, Error>;

    async fn update(&self, user: User) -> Option<Error>;

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error>;
}
