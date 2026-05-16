use starriver_shared_base::{error::RepositoryError, repository::Revision};
use uuid::Uuid;

use crate::user::entity::User;

pub trait UserRepository {
    fn find_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Option<User>, RepositoryError>> + Send;

    fn find_by_id(
        &self,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Option<User>, RepositoryError>> + Send;

    fn insert(&self, user: User) -> impl Future<Output = Result<User, RepositoryError>> + Send;

    fn update(
        &self,
        user: Revision<User>,
    ) -> impl Future<Output = Result<User, RepositoryError>> + Send;

    fn delete(&self, user_id: Uuid) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
