use derive_getters::{Dissolve, Getters};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::security_event::value_object::SecurityEventType;

#[derive(Clone, Debug, PartialEq, Eq, Getters, Dissolve)]
pub struct SecurityEvent {
    id: Uuid,
    event_type: SecurityEventType,
    message: String,
    created_at: OffsetDateTime,
    user_id: Uuid,
}

impl SecurityEvent {
    pub fn new(user_id: Uuid, event_type: SecurityEventType, message: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id,
            event_type,
            message: message.into(),
            created_at: OffsetDateTime::now_utc(),
        }
    }

    /// 是否是某种事件类型
    pub fn is_of_type(&self, event_type: SecurityEventType) -> bool {
        self.event_type == event_type
    }

    /// 是否是错误密码登录尝试
    pub fn is_bad_password_attempt(&self) -> bool {
        self.is_of_type(SecurityEventType::TryLoginWithBadPwd)
    }
}
