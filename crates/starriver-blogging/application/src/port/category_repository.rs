use starriver_blogging_domain::category::entity::Category;
use starriver_shared_base::{
    db::{Executor, Revision},
    error::RepositoryError,
};
use uuid::Uuid;

pub trait CategoryRepository<T: Executor> {
    fn find_by_id(
        &self,
        conn: &T,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Category>, RepositoryError>> + Send;

    fn insert(
        &self,
        conn: &T,
        category: Category,
    ) -> impl Future<Output = Result<Category, RepositoryError>> + Send;

    fn update(
        &self,
        conn: &T,
        category: Revision<Category>,
    ) -> impl Future<Output = Result<Category, RepositoryError>> + Send;

    fn delete(
        &self,
        conn: &T,
        id: Uuid,
    ) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
