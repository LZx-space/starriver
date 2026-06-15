use starriver_shared_base::{
    error::RepositoryError,
    repository::{Executor, Revision},
};
use time::OffsetDateTime;
use uuid::Uuid;

use starriver_identity_domain::security_event::{
    entity::SecurityEvent, value_object::SecurityEventType,
};

pub trait SecurityEventRepository<T: Executor> {
    fn find_by_user_id_since(
        &self,
        conn: &T,
        user_id: Uuid,
        event_type: SecurityEventType,
        since: OffsetDateTime,
    ) -> impl Future<Output = Result<Vec<SecurityEvent>, RepositoryError>> + Send;

    fn insert(
        &self,
        conn: &T,
        event: SecurityEvent,
    ) -> impl Future<Output = Result<SecurityEvent, RepositoryError>> + Send;

    fn update(
        &self,
        conn: &T,
        event: Revision<SecurityEvent>,
    ) -> impl Future<Output = Result<SecurityEvent, RepositoryError>> + Send;

    fn delete(
        &self,
        conn: &T,
        event_id: Uuid,
    ) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
