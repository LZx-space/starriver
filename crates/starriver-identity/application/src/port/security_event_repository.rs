use sea_orm::ConnectionTrait;
use starriver_shared_base::{error::RepositoryError, repository::Revision};
use time::OffsetDateTime;
use uuid::Uuid;

use starriver_identity_domain::security_event::{
    entity::SecurityEvent, value_object::SecurityEventType,
};

pub trait SecurityEventRepository {
    fn find_by_user_id_since<C: ConnectionTrait>(
        &self,
        c: &C,
        user_id: Uuid,
        event_type: SecurityEventType,
        since: OffsetDateTime,
    ) -> impl Future<Output = Result<Vec<SecurityEvent>, RepositoryError>> + Send;

    fn insert<C: ConnectionTrait>(
        &self,
        c: &C,
        event: SecurityEvent,
    ) -> impl Future<Output = Result<SecurityEvent, RepositoryError>> + Send;

    fn update<C: ConnectionTrait>(
        &self,
        c: &C,
        event: Revision<SecurityEvent>,
    ) -> impl Future<Output = Result<SecurityEvent, RepositoryError>> + Send;

    fn delete<C: ConnectionTrait>(
        &self,
        c: &C,
        event_id: Uuid,
    ) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
