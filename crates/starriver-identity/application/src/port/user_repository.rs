use starriver_identity_domain::user::entity::User;
use starriver_shared_base::{
    error::RepositoryError,
    repository::{Executor, Revision},
};
use uuid::Uuid;

pub trait UserRepository<C: Executor> {
    fn find_by_username(
        &self,
        conn: &C,
        username: &str,
    ) -> impl Future<Output = Result<Option<User>, RepositoryError>> + Send;

    fn insert(
        &self,
        conn: &C,
        user: User,
    ) -> impl Future<Output = Result<User, RepositoryError>> + Send;

    fn update(
        &self,
        conn: &C,
        user: Revision<User>,
    ) -> impl Future<Output = Result<User, RepositoryError>> + Send;

    fn delete(
        &self,
        conn: &C,
        user_id: Uuid,
    ) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
