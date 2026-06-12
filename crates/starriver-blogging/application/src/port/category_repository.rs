use sea_orm::ConnectionTrait;
use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::{error::RepositoryError, repository::Revision};
use uuid::Uuid;

pub trait CategoryRepository {
    fn find_by_id<C: ConnectionTrait>(
        &self,
        conn: &C,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Category>, RepositoryError>> + Send;

    fn insert<C: ConnectionTrait>(
        &self,
        conn: &C,
        category: Category,
    ) -> impl Future<Output = Result<Category, RepositoryError>> + Send;

    fn update<C: ConnectionTrait>(
        &self,
        conn: &C,
        category: Revision<Category>,
    ) -> impl Future<Output = Result<Category, RepositoryError>> + Send;

    fn delete<C: ConnectionTrait>(
        &self,
        conn: &C,
        id: Uuid,
    ) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
