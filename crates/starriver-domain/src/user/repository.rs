use crate::user::entity::User;
use starriver_infrastructure::error::error::AppError;

pub trait UserRepository {
    fn insert(&self, user: User) -> impl Future<Output = Result<User, AppError>> + Send;

    fn update(&self, user: User) -> impl Future<Output = Result<User, AppError>> + Send;

    fn find_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Option<User>, AppError>> + Send;
}
