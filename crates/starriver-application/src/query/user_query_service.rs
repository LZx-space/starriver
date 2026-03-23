use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use starriver_infrastructure::error::ApiError;

use crate::db::user_do::{self, Entity};

pub trait UserQueryService {
    async fn find_by_email(&self, email: &str) -> Result<bool, ApiError>;
}

pub struct DefaultUserQueryService {
    pub conn: &'static DatabaseConnection,
}

impl UserQueryService for DefaultUserQueryService {
    async fn find_by_email(&self, email: &str) -> Result<bool, ApiError> {
        let user = Entity::find()
            .filter(user_do::Column::Email.eq(email.to_string()))
            .one(self.conn)
            .await?;
        Ok(user.is_some())
    }
}
