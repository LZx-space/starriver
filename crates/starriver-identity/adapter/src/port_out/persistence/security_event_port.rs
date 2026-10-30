use std::convert::Infallible;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, ConnectionTrait, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use starriver_identity_application::{
    dto::user_dto::{input::SecurityEventCmd, output::SecurityEventPage},
    port::security_event_port::{SecurityEventCursor, SecurityEventFilter, SecurityEventPort},
};
use starriver_shared_base::{db::Executor, error::QueryError};
use time::OffsetDateTime;
use tracing::error;
use uuid::Uuid;

use crate::port_out::persistence::{
    dto::security_event_row::SecurityEventRow,
    po::security_event_po::{ActiveModel, Column, Entity, SecurityEventTypePo},
};

pub struct DefaultSecurityEventPort;

impl<T> SecurityEventPort<T> for DefaultSecurityEventPort
where
    T: Executor + ConnectionTrait,
{
    async fn list(
        &self,
        conn: &T,
        filter: SecurityEventFilter,
    ) -> Result<SecurityEventPage, QueryError> {
        let limit = filter.limit.clamp(1, 100) as usize;

        let mut cond = Condition::all();
        if let Some(user_id) = filter.user_id {
            cond = cond.add(Column::UserId.eq(user_id));
        }
        if let Some(event_type) = filter.event_type {
            cond = cond.add(Column::EventType.eq(SecurityEventTypePo::from(event_type)));
        }
        if let Some(from) = filter.from {
            cond = cond.add(Column::CreatedAt.gte(from));
        }
        if let Some(to) = filter.to {
            cond = cond.add(Column::CreatedAt.lte(to));
        }
        if let Some(after) = filter.after {
            cond = cond.add(
                Condition::any()
                    .add(Column::CreatedAt.lt(after.occurred_at))
                    .add(
                        Condition::all()
                            .add(Column::CreatedAt.eq(after.occurred_at))
                            .add(Column::Id.lt(after.id)),
                    ),
            );
        }

        let rows = Entity::find()
            .select_only()
            .column(Column::Id)
            .column(Column::UserId)
            .column(Column::Username)
            .column(Column::EventType)
            .column(Column::CreatedAt)
            .column_as(Column::CreatedAt, "occurred_at")
            .filter(cond)
            .order_by_desc(Column::CreatedAt)
            .order_by_desc(Column::Id)
            .limit(limit as u64 + 1)
            .into_model::<SecurityEventRow>()
            .all(conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))?;

        let has_more = rows.len() > limit;
        let next_cursor = if has_more {
            rows.get(limit - 1)
                .map(|row| SecurityEventCursor {
                    occurred_at: row.occurred_at,
                    id: row.id,
                })
                .map(|cursor| cursor.encode())
        } else {
            None
        };
        let items = rows
            .into_iter()
            .take(limit)
            .map(Into::into)
            .collect::<Vec<_>>();

        Ok(SecurityEventPage { items, next_cursor })
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
