use crate::user::entity::User;
use anyhow::Error;

pub trait UserRepository {
    fn insert(&self, user: User) -> impl Future<Output = Result<User, Error>> + Send;

    fn update(&self, user: User) -> impl Future<Output = Option<Error>> + Send;

    fn find_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Option<User>, Error>> + Send;
}
