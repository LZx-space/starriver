use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, EntityTrait, PaginatorTrait, QueryOrder,
    QuerySelect,
};
use starriver_identity_application::{
    dto::user_dto::{
        req::{SecurityEventCmd, SecurityEventType},
        res::SecurityEventDto,
    },
    port::security_event_port::SecurityEventPort,
};
use starriver_shared_base::{
    dto::{PageQuery, PageResult},
    error::{QueryError, RepositoryError},
};
use starriver_shared_framework::{
    db::{DefaultConnection, DefaultTransaction},
    error_mapping::db_2_repo_error,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::security_event_po::{ActiveModel, Column, Entity};

pub struct DefaultSecurityEventPort;

impl DefaultSecurityEventPort {
    async fn paginate(
        &self,
        conn: &impl ConnectionTrait,
        q: PageQuery,
    ) -> Result<PageResult<SecurityEventDto>, QueryError> {
        let events = Entity::find()
            .order_by_desc(Column::CreatedAt)
            .offset(q.page * q.page_size)
            .limit(q.page_size)
            .all(conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?
            .into_iter()
            .map(|e| SecurityEventDto {
                id: e.id,
                event_type: SecurityEventType::from(e.event_type).to_string(),
                occurred_at: e.created_at,
            })
            .collect::<Vec<_>>();
        let record_total = Entity::find()
            .select_only()
            .column(Column::Id)
            .count(conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?;
        Ok(PageResult::new(q.page, q.page_size, record_total, events))
    }

    async fn insert(
        &self,
        conn: &impl ConnectionTrait,
        event: SecurityEventCmd,
    ) -> Result<(), RepositoryError> {
        ActiveModel {
            id: Set(Uuid::now_v7()),
            event_type: Set(event.event_type.into()),
            message: Set(event.payload),
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: Set(None),
            user_id: Set(event.user_id),
        }
        .insert(conn)
        .await
        .map_err(db_2_repo_error)
        .map(|_| ())
    }
}

impl SecurityEventPort<DefaultConnection> for DefaultSecurityEventPort {
    async fn paginate(
        &self,
        conn: &DefaultConnection,
        q: PageQuery,
    ) -> Result<PageResult<SecurityEventDto>, QueryError> {
        self.paginate(conn, q).await
    }

    async fn insert(
        &self,
        conn: &DefaultConnection,
        event: SecurityEventCmd,
    ) -> Result<(), RepositoryError> {
        self.insert(conn, event).await
    }
}

impl SecurityEventPort<DefaultTransaction> for DefaultSecurityEventPort {
    async fn paginate(
        &self,
        conn: &DefaultTransaction,
        q: PageQuery,
    ) -> Result<PageResult<SecurityEventDto>, QueryError> {
        self.paginate(conn, q).await
    }

    async fn insert(
        &self,
        conn: &DefaultTransaction,
        event: SecurityEventCmd,
    ) -> Result<(), RepositoryError> {
        self.insert(conn, event).await
    }
}
