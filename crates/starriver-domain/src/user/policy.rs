use crate::user::{entity::SecurityEvent, value_object::SecurityEventType};
use starriver_infrastructure::service::config_service::UserPolicy;
use std::ops::Add;
use std::time::Duration;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct UserLockPolicy {
    bad_password_window_mins: u64,
    max_bad_password_attempts: usize,
}

impl UserLockPolicy {
    pub fn new(cfg: &UserPolicy) -> Self {
        Self {
            bad_password_window_mins: cfg.bad_password_window_mins,
            max_bad_password_attempts: cfg.max_bad_password_attempts,
        }
    }

    /// 依据尝试密码次数来锁定账户
    pub fn should_lock(&self, security_events: &[SecurityEvent]) -> bool {
        let now = OffsetDateTime::now_utc();
        security_events
            .iter()
            .filter(|e| {
                let event_type = e.event_type();
                let create_at = e.created_at();
                matches!(event_type, SecurityEventType::TryLoginWithBadPwd)
                    && create_at.add(Duration::from_mins(self.bad_password_window_mins)) >= now
            })
            .count()
            > self.max_bad_password_attempts
    }
}
