use starriver_infrastructure::{error::ApiError, model::aggregate_revision::Revision};
use uuid::Uuid;

use crate::category::entity::Category;

pub trait CategoryRepository {
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Category>, ApiError>> + Send;

    fn list(&self) -> impl Future<Output = Result<Vec<Category>, ApiError>> + Send;

    fn insert(&self, category: Category)
    -> impl Future<Output = Result<Category, ApiError>> + Send;

    fn update(
        &self,
        category: Revision<Category>,
    ) -> impl Future<Output = Result<Category, ApiError>> + Send;

    fn delete(&self, id: Uuid) -> impl Future<Output = Result<bool, ApiError>> + Send;
}
