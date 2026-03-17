use crate::user::{entity::SecurityEvent, value_object::SecurityEventType};
use starriver_infrastructure::error::{ApiError, Cause};
use std::ops::Add;
use time::{Duration, OffsetDateTime};

pub struct PasswordSpecification {
    min_length: u8,
    max_length: u8,
    accumulate_bad_password_times_duration: Duration,
    max_bad_password_times: usize,
}

impl PasswordSpecification {
    pub fn validate_new_password(&self, password: &str) -> Result<(), ApiError> {
        if password.len() < self.min_length as usize {
            return Err(ApiError::new(Cause::ClientBadRequest, "password too short"));
        }
        if password.len() > self.max_length as usize {
            return Err(ApiError::new(Cause::ClientBadRequest, "password too long"));
        }
        Ok(())
    }

    /// 依据尝试密码次数来锁定账户
    pub fn lock_if_try_exceeded(
        &mut self,
        login_events: Vec<SecurityEvent>,
    ) -> Result<bool, ApiError> {
        let now = OffsetDateTime::now_utc();
        let bad_pwd_times = login_events
            .iter()
            .filter(|e| SecurityEventType::TryLoginWithBadPwd.eq(&e.event_type))
            .filter(|e| {
                e.created_at
                    .add(self.accumulate_bad_password_times_duration)
                    >= now
            })
            .count();
        if bad_pwd_times == 0 {
            return Err(ApiError::new(
                Cause::ClientBadRequest,
                "bad password never happened",
            ));
        }
        Ok(bad_pwd_times > self.max_bad_password_times)
    }
}

impl Default for PasswordSpecification {
    fn default() -> Self {
        PasswordSpecification {
            min_length: 6,
            max_length: 20,
            accumulate_bad_password_times_duration: Duration::minutes(10),
            max_bad_password_times: 5,
        }
    }
}
