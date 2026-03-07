use crate::user::entity::LoginEvent;
use starriver_infrastructure::error::error::{ApiError, Cause};
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
