use derive_getters::{Dissolve, Getters};

use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::user::{
    policy::BadPasswordPolicy,
    value_object::{Email, HashedPassword, LifeCycle, Username},
};

// -----Aggregate Root User------------------------------------------------------
/// The user aggregate. User is the aggregate root.
#[derive(Clone, Debug, Getters, Dissolve)]
pub struct User {
    id: Uuid,
    username: Username,
    password: HashedPassword,
    email: Email,
    life_cycle: LifeCycle,
    password_locked_until: Option<OffsetDateTime>,
    password_window_start: Option<OffsetDateTime>,
    password_attempts: u8,
}

impl User {
    pub(crate) fn new(
        id: Uuid,
        username: Username,
        password: HashedPassword,
        email: Email,
        life_cycle: LifeCycle,
    ) -> Self {
        Self {
            id,
            username,
            password,
            email,
            life_cycle,
            password_locked_until: None,
            password_window_start: None,
            password_attempts: 0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_repo(
        id: Uuid,
        username: String,
        password: String,
        email: String,
        life_cycle: LifeCycle,
        password_locked_until: Option<OffsetDateTime>,
        password_window_start: Option<OffsetDateTime>,
        password_attempts: u8,
    ) -> Self {
        let username = Username::from_repo(username);
        let password = HashedPassword::from_repo(password);
        let email = Email::from_repo(email);
        Self {
            id,
            username,
            password,
            email,
            life_cycle,
            password_locked_until,
            password_window_start,
            password_attempts,
        }
    }

    pub fn activate(&mut self) {
        self.life_cycle = LifeCycle::Active;
    }

    pub fn disable(&mut self) {
        self.life_cycle = LifeCycle::Disabled;
    }

    pub fn delete(&mut self) {
        self.life_cycle = LifeCycle::Deleted;
    }

    pub fn is_locked(&self) -> bool {
        self.password_locked_until
            .is_some_and(|time| time > OffsetDateTime::now_utc())
    }

    pub fn unlock(&mut self) {
        self.password_locked_until = None;
        self.password_window_start = None;
        self.password_attempts = 0;
    }

    pub fn record_bad_password_and_attempt_lock(&mut self, policy: &BadPasswordPolicy) {
        let now = OffsetDateTime::now_utc();
        let window_duration = Duration::minutes(policy.window_minutes.into());

        let within_window = self
            .password_window_start
            .is_some_and(|start| now - start <= window_duration);

        if within_window {
            // 窗口内：计数递增
            self.password_attempts += 1;
        } else {
            // 窗口不存在或已过期：开启新窗口，计数从1开始
            self.password_window_start = Some(now);
            self.password_attempts = 1;
        }

        if self.password_attempts >= policy.max_attempts {
            let lockout_duration = Duration::minutes(policy.lockout_minutes.into());
            let lockout_until = now + lockout_duration;
            self.password_locked_until = Some(lockout_until);
        }
    }

    pub fn change_password(&mut self, new_pwd: HashedPassword) {
        self.password = new_pwd;
    }
}
