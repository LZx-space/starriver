use crate::user::entity::LoginEvent;
use crate::user::value_object::Password;
use std::ops::Add;
use time::{Duration, OffsetDateTime};

pub(crate) struct PasswordSpecification {
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

    pub fn during_validity_duration(&self, pwd: &Password) -> bool {
        pwd.set_at().add(self.validity_duration) > OffsetDateTime::now_utc()
    }

    pub fn set_account_locked(&mut self, login_events: Vec<LoginEvent>) -> bool {
        let now = OffsetDateTime::now_utc();
        login_events
            .iter()
            .filter(|e| e.login_at.add(self.accumulate_bad_password_times_duration) >= now)
            .count()
            > self.max_bad_password_times
    }
}
