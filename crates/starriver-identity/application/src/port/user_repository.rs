use starriver_identity_domain::user::{
    entity::User,
    value_object::{Email, Username},
};
use starriver_shared_base::{
    db::{Executor, Revision},
    error::RepositoryError,
};
use uuid::Uuid;

pub trait UserRepository<T: Executor> {
    fn find_by_username(
        &self,
        conn: &T,
        username: &Username,
    ) -> impl Future<Output = Result<Option<User>, RepositoryError>> + Send;

    fn find_by_email(
        &self,
        conn: &T,
        email: &Email,
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
