use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use starriver_identity_application::port::user_query::UserQuery;
use starriver_shared_base::error::QueryError;
use uuid::Uuid;

use crate::port_out::persistence::po::user_po::{self, Entity};

pub struct DefaultUserQuery;

impl UserQuery for DefaultUserQuery {
    async fn exists_by_email<C: ConnectionTrait>(
        &self,
        conn: &C,
        email: &str,
    ) -> Result<bool, QueryError> {
        Entity::find()
            .select_only()
            .filter(user_po::Column::Email.eq(email.to_string()))
            .exists(conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))
    }

    async fn find_email_by_user_id<C: ConnectionTrait>(
        &self,
        conn: &C,
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
