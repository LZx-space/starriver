use crate::user::entity::User;
use starriver_infrastructure::error::error::ApiError;
use uuid::Uuid;

pub trait UserRepository {
    fn find_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Option<User>, ApiError>> + Send;

    fn insert(&self, user: User) -> impl Future<Output = Result<User, ApiError>> + Send;

    fn update(&self, user: User) -> impl Future<Output = Result<User, ApiError>> + Send;

    fn delete(&self, user_id: Uuid) -> impl Future<Output = Result<bool, ApiError>> + Send;
}
