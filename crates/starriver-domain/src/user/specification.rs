use crate::user::entity::LoginEvent;
use crate::user::value_object::Password;
use std::ops::Add;
use time::{Duration, OffsetDateTime};

pub struct PasswordSpecification {
    validity_duration: Duration,
    accumulate_bad_password_times_duration: Duration,
    max_bad_password_times: usize,
}

impl PasswordSpecification {
    pub fn new(
        validity_duration: Duration,
        accumulate_bad_password_times_duration: Duration,
        max_bad_password_times: usize,
    ) -> Self {
        PasswordSpecification {
            validity_duration,
            accumulate_bad_password_times_duration,
            max_bad_password_times,
        }
    }

    /// 检查密码是否在有效期内
    pub fn is_within_validity(&self, pwd: &Password) -> bool {
        pwd.set_at().add(self.validity_duration) > OffsetDateTime::now_utc()
    }

    /// 依据尝试密码次数来锁定账户
    pub fn lock_if_attempts_exceeded(&mut self, login_events: Vec<LoginEvent>) -> bool {
        let now = OffsetDateTime::now_utc();
        login_events
            .iter()
            .filter(|e| {
                !e.is_sccuess && e.try_at.add(self.accumulate_bad_password_times_duration) >= now
            })
            .count()
            > self.max_bad_password_times
    }
}
