use starriver_identity_domain::user::entity::User;
use starriver_shared_base::{
    error::RepositoryError,
    repository::{Executor, Revision},
};
use uuid::Uuid;

pub trait UserRepository<T: Executor> {
    fn find_by_username(
        &self,
        conn: &T,
        username: &str,
    ) -> impl Future<Output = Result<Option<User>, RepositoryError>> + Send;

    fn insert(
        &self,
        conn: &T,
        user: User,
    ) -> impl Future<Output = Result<User, RepositoryError>> + Send;

    fn update(
        &self,
        conn: &T,
        user: Revision<User>,
    ) -> impl Future<Output = Result<User, RepositoryError>> + Send;

    fn delete(
        &self,
        conn: &T,
        user_id: Uuid,
    ) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
