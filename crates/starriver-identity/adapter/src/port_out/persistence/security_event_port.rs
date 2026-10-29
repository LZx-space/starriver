use std::convert::Infallible;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, EntityTrait, PaginatorTrait, QueryOrder,
    QuerySelect,
};
use starriver_identity_application::{
    dto::user_dto::{req::SecurityEventCmd, res::SecurityEventDto},
    port::security_event_port::SecurityEventPort,
};
use starriver_shared_base::{
    db::Executor,
    dto::{PageQuery, PageResult},
    error::QueryError,
};
use time::OffsetDateTime;
use tracing::error;
use uuid::Uuid;

use crate::port_out::persistence::{
    dto::security_event_row::SecurityEventRow,
    po::security_event_po::{ActiveModel, Column, Entity},
};

pub struct DefaultSecurityEventPort;

impl<T> SecurityEventPort<T> for DefaultSecurityEventPort
where
    T: Executor + ConnectionTrait,
{
    async fn paginate(
        &self,
        conn: &T,
        q: PageQuery,
    ) -> Result<PageResult<SecurityEventDto>, QueryError> {
        let paginator = Entity::find()
            .select_only()
            .column(Column::Id)
            .column(Column::UserId)
            .column(Column::Username)
            .column(Column::EventType)
            .column(Column::CreatedAt)
            .column_as(Column::CreatedAt, "occurred_at")
            .order_by_desc(Column::CreatedAt)
            .order_by_desc(Column::Id)
            .into_model::<SecurityEventRow>()
            .paginate(conn, q.page_size);
        let events = paginator
            .fetch_page(q.page)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?
            .into_iter()
            .map(|e| e.into())
            .collect::<Vec<_>>();
        let total_items = paginator
            .num_items()
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?;
        Ok(PageResult::new(q.page, q.page_size, total_items, events))
    }

    async fn insert(&self, conn: &T, event: SecurityEventCmd) -> Result<(), Infallible> {
        let (user_id, username, event_type, payload) = event.into_parts();
        let result = ActiveModel {
            id: Set(Uuid::now_v7()),
            user_id: Set(user_id),
            username: Set(username),
            event_type: Set(event_type.into()),
            message: Set(payload.into_owned()),
            created_at: Set(OffsetDateTime::now_utc()),
        }
        .insert(conn)
        .await;

        if let Err(e) = result {
            error!(error=%e, "insert security event failed");
        }
        Ok(())
    }
}
