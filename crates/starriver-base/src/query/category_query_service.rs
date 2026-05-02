use sea_orm::{DatabaseConnection, EntityOrSelect, EntityTrait};
use uuid::Uuid;

use crate::{db::category_do::Entity, dto::category_dto::res::CategoryDetail, error::ApiError};

pub trait CategoryQueryService {
    fn list(&self) -> impl Future<Output = Result<Vec<CategoryDetail>, ApiError>>;

    fn find(&self, id: Uuid) -> impl Future<Output = Result<Option<CategoryDetail>, ApiError>>;
}

pub struct DefaultCategoryQueryService {
    pub conn: DatabaseConnection,
}

impl CategoryQueryService for DefaultCategoryQueryService {
    async fn list(&self) -> Result<Vec<CategoryDetail>, ApiError> {
        Entity::find()
            .select()
            .into_model::<CategoryDetail>()
            .all(&self.conn)
            .await
            .map_err(ApiError::from)
    }

    async fn find(&self, id: Uuid) -> Result<Option<CategoryDetail>, ApiError> {
        Entity::find_by_id(id)
            .select()
            .into_model::<CategoryDetail>()
            .one(&self.conn)
            .await
            .map_err(ApiError::from)
    }
}
