use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use starriver_identity_application::port_out::user_query_port::UserQueryPort;
use starriver_shared_base::error::QueryError;

use crate::port_out::persistence::po::user_po::{self, Entity};

pub struct DefaultUserQueryPort {
    pub conn: DatabaseConnection,
}

impl UserQueryPort for DefaultUserQueryPort {
    async fn exists_by_email(&self, email: &str) -> Result<bool, QueryError> {
        Entity::find()
            .select_only()
            .filter(user_po::Column::Email.eq(email.to_string()))
            .exists(&self.conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))
    }
}
