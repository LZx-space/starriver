use sea_orm::{DatabaseConnection, EntityOrSelect, EntityTrait};
use starriver_infrastructure::error::ApiError;
use uuid::Uuid;

use crate::{category_dto::res::CategoryDetail, db::category_do::Entity};

pub trait CategoryQueryService {
    async fn list(&self) -> Result<Vec<CategoryDetail>, ApiError>;

    async fn find(&self, id: Uuid) -> Result<Option<CategoryDetail>, ApiError>;
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
