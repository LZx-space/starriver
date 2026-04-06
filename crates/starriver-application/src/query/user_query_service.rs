use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use starriver_infrastructure::error::ApiError;

use crate::db::user_do::{self, Entity};

pub trait UserQueryService {
    async fn exists_by_email(&self, email: &str) -> Result<bool, ApiError>;
}

pub struct DefaultUserQueryService {
    pub conn: DatabaseConnection,
}

impl UserQueryService for DefaultUserQueryService {
    async fn exists_by_email(&self, email: &str) -> Result<bool, ApiError> {
        Entity::find()
            .select_only()
            .filter(user_do::Column::Email.eq(email.to_string()))
            .exists(&self.conn)
            .await
            .map_err(ApiError::from)
    }
}
