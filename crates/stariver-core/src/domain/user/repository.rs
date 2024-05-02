use anyhow::Error;

use crate::domain::user::aggregate::User;

#[trait_variant::make(HttpService: Send)]
pub trait UserRepository {
    async fn insert(&self, user: User) -> Result<User, Error>;

    async fn update(&self, user: User) -> Option<Error>;

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error>;
}
