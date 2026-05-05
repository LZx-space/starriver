use derive_getters::{Dissolve, Getters};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::aggregate::security_event_value_object::SecurityEventType;

#[derive(Clone, Debug, PartialEq, Eq, Getters, Dissolve)]
pub struct SecurityEvent {
    id: Uuid,
    user_id: Uuid,
    event_type: SecurityEventType,
    message: String,
    created_at: OffsetDateTime,
}

impl SecurityEvent {
    pub(super) fn new(user_id: Uuid, event_type: SecurityEventType, message: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id,
            event_type,
            message: message.into(),
            created_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn from_repo(
        id: Uuid,
        user_id: Uuid,
        event_type: SecurityEventType,
        message: String,
        created_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            user_id,
            event_type,
            message,
            created_at,
        }
    }
}
