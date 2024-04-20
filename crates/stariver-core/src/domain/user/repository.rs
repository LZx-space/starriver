use std::fmt::Error;

use crate::domain::user::aggregate::User;

#[trait_variant::make(HttpService: Send)]
pub trait UserRepository {
    async fn insert(&self, user: User) -> Result<bool, Error>;

    async fn update(&self, user: User) -> Result<bool, Error>;

    async fn find_by_username(&self, username: &str) -> Result<User, Error>;
}
