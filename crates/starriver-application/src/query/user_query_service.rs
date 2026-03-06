use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use starriver_infrastructure::error::error::ApiError;

use crate::{
    db::user_do::{Column, Entity},
    user_dto::SecurityUser,
};

pub trait UserQueryService {
    fn find_by_username(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Option<SecurityUser>, ApiError>> + Send;
}

pub struct DefaultUserQueryService {
    pub conn: &'static DatabaseConnection,
}

impl UserQueryService for DefaultUserQueryService {
    async fn find_by_username(&self, username: &str) -> Result<Option<SecurityUser>, ApiError> {
        Entity::find()
            .filter(Column::Username.eq(username))
            .one(self.conn)
            .await?
            .map(|m| {
                let username = m.username;
                let password = m.password;
                Ok(SecurityUser { username, password })
            })
            .transpose()
    }
}
