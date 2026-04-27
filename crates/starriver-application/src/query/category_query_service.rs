use sea_orm::{DatabaseConnection, EntityOrSelect, EntityTrait};
use starriver_infrastructure::error::ApiError;

use crate::{category_dto::res::CategoryDetail, db::category_do::Entity};

pub trait CategoryQueryService {
    async fn list(&self) -> Result<Vec<CategoryDetail>, ApiError>;
}

pub struct DefaultCategoryQueryService {
    pub conn: DatabaseConnection,
}

impl CategoryQueryService for DefaultCategoryQueryService {
    async fn list(&self) -> Result<Vec<CategoryDetail>, ApiError> {
        let x = Entity::find()
            .select()
            .into_model::<CategoryDetail>()
            .all(&self.conn)
            .await?;
        Ok(x)
    }
}
