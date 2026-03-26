use crate::user::{entity::SecurityEvent, value_object::SecurityEventType};
use std::ops::Add;
use time::{Duration, OffsetDateTime};

pub struct UserPolicy {
    accumulate_bad_password_times_duration: Duration,
    max_bad_password_times: usize,
}

impl UserPolicy {
    /// 依据尝试密码次数来锁定账户
    pub fn should_lock(&self, security_events: &[SecurityEvent]) -> bool {
        let now = OffsetDateTime::now_utc();
        security_events
            .iter()
            .filter(|e| SecurityEventType::TryLoginWithBadPwd.eq(&e.event_type))
            .filter(|e| {
                e.created_at
                    .add(self.accumulate_bad_password_times_duration)
                    >= now
            })
            .count()
            > self.max_bad_password_times
    }
}

impl Default for UserPolicy {
    fn default() -> Self {
        UserPolicy {
            accumulate_bad_password_times_duration: Duration::minutes(10),
            max_bad_password_times: 5,
        }
    }
}
