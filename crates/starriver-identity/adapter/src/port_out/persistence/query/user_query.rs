use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};
use starriver_identity_application::port::user_query::UserQuery;
use starriver_shared_base::error::QueryError;
use starriver_shared_framework::repository::DefaultConnection;
use uuid::Uuid;

use crate::port_out::persistence::po::user_po::{self, Entity};

pub struct DefaultUserQuery;

impl UserQuery<DefaultConnection> for DefaultUserQuery {
    async fn exists_by_email(
        &self,
        conn: &DefaultConnection,
        email: &str,
    ) -> Result<bool, QueryError> {
        Entity::find()
            .select_only()
            .filter(user_po::Column::Email.eq(email.to_string()))
            .exists(conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))
    }

    async fn find_email_by_user_id(
        &self,
        conn: &DefaultConnection,
        user_id: Uuid,
    ) -> Result<Option<String>, QueryError> {
        Entity::find()
            .select_only()
            .filter(user_po::Column::Id.eq(user_id))
            .column(user_po::Column::Email)
            .one(conn)
            .await
            .map(|e| e.map(|e| e.email))
            .map_err(|e| QueryError::DbError(e.to_string()))
    }
}
