use starriver_shared_base::{error::RepositoryError, repository::Revision};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::security_event::{entity::SecurityEvent, value_object::SecurityEventType};

pub trait SecurityEventRepository {
    fn find_by_user_id_since(
        &self,
        user_id: Uuid,
        event_type: SecurityEventType,
        since: OffsetDateTime,
    ) -> impl Future<Output = Result<Vec<SecurityEvent>, RepositoryError>> + Send;

    fn insert(
        &self,
        event: SecurityEvent,
    ) -> impl Future<Output = Result<SecurityEvent, RepositoryError>> + Send;

    fn update(
        &self,
        event: Revision<SecurityEvent>,
    ) -> impl Future<Output = Result<SecurityEvent, RepositoryError>> + Send;

    fn delete(&self, event_id: Uuid) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
