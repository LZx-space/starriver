use sea_orm::{
    ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    sea_query::NullOrdering,
};
use starriver_identity_application::{
    dto::user_dto::res::UserDetailDto, port::user_query::UserQuery,
};
use starriver_shared_base::{
    dto::{PageQuery, PageResult},
    error::QueryError,
};
use starriver_shared_framework::db::DefaultConnection;
use uuid::Uuid;

use crate::port_out::persistence::po::user_po::{self, Column, Entity};

pub struct DefaultUserQuery;

impl UserQuery<DefaultConnection> for DefaultUserQuery {
    async fn paginate(
        &self,
        conn: &DefaultConnection,
        q: PageQuery,
    ) -> Result<PageResult<UserDetailDto>, QueryError> {
        let users = Entity::find()
            .order_by_with_nulls(Column::UpdatedAt, Order::Desc, NullOrdering::Last)
            .offset(q.page * q.page_size)
            .limit(q.page_size)
            .all(conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?
            .into_iter()
            .map(|e| UserDetailDto {
                id: e.id,
                username: e.username,
                email: e.email,
                life_cycle: e.life_cycle.into(),
                password_locked_until: e.password_locked_until,
                password_window_start: e.password_window_start,
                password_attempts: e.password_attempts,
            })
            .collect::<Vec<_>>();
        let record_total = Entity::find()
            .select_only()
            .column(Column::Id)
            .count(conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?;
        Ok(PageResult::new(q.page, q.page_size, record_total, users))
    }

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
