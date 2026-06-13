use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
};
use starriver_identity_application::port::security_event_repository::SecurityEventRepository;
use starriver_identity_domain::security_event::{
    entity::SecurityEvent, value_object::SecurityEventType,
};
use starriver_shared_base::{error::RepositoryError, repository::Revision};
use starriver_shared_framework::error_mapping::db_2_repo_error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::security_event_po::{
    ActiveModel, Column, Entity, SecurityEventTypePo,
};

pub struct DefaultSecurityEventRepository;

impl SecurityEventRepository for DefaultSecurityEventRepository {
    async fn find_by_user_id_since<C: ConnectionTrait>(
        &self,
        conn: &C,
        user_id: Uuid,
        event_type: SecurityEventType,
        since: OffsetDateTime,
    ) -> Result<Vec<SecurityEvent>, RepositoryError> {
        let event_type: SecurityEventTypePo = event_type.into();
        let security_events = Entity::find()
            .filter(Column::UserId.eq(user_id))
            .filter(Column::EventType.eq(event_type))
            .filter(Column::CreatedAt.gte(since))
            .all(conn)
            .await
            .map_err(db_2_repo_error)?
            .into_iter()
            .map(|e| {
                SecurityEvent::from_repo(
                    e.id,
                    e.event_type.into(),
                    e.message.clone(),
                    e.created_at,
                    e.user_id,
                )
            })
            .collect::<Vec<_>>();
        Ok(security_events)
    }

    async fn insert<C: ConnectionTrait>(
        &self,
        conn: &C,
        event: SecurityEvent,
    ) -> Result<SecurityEvent, RepositoryError> {
        let fields = event.dissolve();
        ActiveModel {
            id: Set(fields.0),
            event_type: Set(fields.1.into()),
            message: Set(fields.2),
            created_at: Set(fields.3),
            user_id: Set(fields.4),
            updated_at: Set(None),
        }
        .insert(conn)
        .await
        .map(|e| {
            SecurityEvent::from_repo(
                e.id,
                e.event_type.into(),
                e.message,
                e.created_at,
                e.user_id,
            )
        })
        .map_err(db_2_repo_error)
    }

    async fn update<C: ConnectionTrait>(
        &self,
        conn: &C,
        event: Revision<SecurityEvent>,
    ) -> Result<SecurityEvent, RepositoryError> {
        let (original, modified) = event.dissolve();
        let original_fields = original.dissolve();
        let modified_fields = modified.dissolve();
        ActiveModel {
            id: Set(original_fields.0),
            event_type: Set(modified_fields.1.into()),
            message: Set(modified_fields.2),
            created_at: Set(modified_fields.3),
            user_id: Set(modified_fields.4),
            updated_at: Set(None),
        }
        .update(conn)
        .await
        .map(|e| {
            SecurityEvent::from_repo(
                e.id,
                e.event_type.into(),
                e.message,
                e.created_at,
                e.user_id,
            )
        })
        .map_err(db_2_repo_error)
    }

    async fn delete<C: ConnectionTrait>(
        &self,
        conn: &C,
        event_id: Uuid,
    ) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(event_id)
            .exec(conn)
            .await
            .map(|e| e.rows_affected > 0)
            .map_err(db_2_repo_error)
    }
}
