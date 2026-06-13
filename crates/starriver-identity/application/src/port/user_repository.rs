use sea_orm::ConnectionTrait;
use starriver_identity_domain::user::entity::User;
use starriver_shared_base::{error::RepositoryError, repository::Revision};
use uuid::Uuid;

pub trait UserRepository {
    fn find_by_username<C: ConnectionTrait>(
        &self,
        conn: &C,
        username: &str,
    ) -> impl Future<Output = Result<Option<User>, RepositoryError>> + Send;

    fn insert<C: ConnectionTrait>(
        &self,
        conn: &C,
        user: User,
    ) -> impl Future<Output = Result<User, RepositoryError>> + Send;

    fn update<C: ConnectionTrait>(
        &self,
        conn: &C,
        user: Revision<User>,
    ) -> impl Future<Output = Result<User, RepositoryError>> + Send;

    fn delete<C: ConnectionTrait>(
        &self,
        conn: &C,
        user_id: Uuid,
    ) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
