use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use starriver_identity_application::port::user_query::UserQuery;
use starriver_shared_base::error::QueryError;
use uuid::Uuid;

use crate::port_out::persistence::po::user_po::{self, Entity};

pub struct DefaultUserQuery {
    pub conn: DatabaseConnection,
}

impl UserQuery for DefaultUserQuery {
    async fn exists_by_email(&self, email: &str) -> Result<bool, QueryError> {
        Entity::find()
            .select_only()
            .filter(user_po::Column::Email.eq(email.to_string()))
            .exists(&self.conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))
    }

    async fn find_email_by_user_id(&self, user_id: Uuid) -> Result<Option<String>, QueryError> {
        Entity::find()
            .select_only()
            .filter(user_po::Column::Id.eq(user_id))
            .column(user_po::Column::Email)
            .one(&self.conn)
            .await
            .map(|e| e.map(|e| e.email))
            .map_err(|e| QueryError::DbError(e.to_string()))
    }
}
