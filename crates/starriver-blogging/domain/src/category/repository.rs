use starriver_shared_base::{error::RepositoryError, repository::Revision};
use uuid::Uuid;

use crate::category::entity::Category;

pub trait CategoryRepository {
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Category>, RepositoryError>> + Send;

    fn insert(
        &self,
        category: Category,
    ) -> impl Future<Output = Result<Category, RepositoryError>> + Send;

    fn update(
        &self,
        category: Revision<Category>,
    ) -> impl Future<Output = Result<Category, RepositoryError>> + Send;

    fn delete(&self, id: Uuid) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
