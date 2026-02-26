use crate::user::entity::User;
use starriver_infrastructure::error::error::ApiError;

pub trait UserRepository {
    fn insert(&self, user: User) -> impl Future<Output = Result<User, ApiError>> + Send;

    fn update(&self, user: User) -> impl Future<Output = Result<User, ApiError>> + Send;

    fn find_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Option<User>, ApiError>> + Send;
}
