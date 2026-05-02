use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};

use crate::{
    db::user_do::{self, Entity},
    error::ApiError,
};

pub trait UserQueryService {
    fn exists_by_email(&self, email: &str) -> impl Future<Output = Result<bool, ApiError>>;
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
