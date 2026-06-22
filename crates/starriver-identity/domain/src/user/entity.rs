use derive_getters::{Dissolve, Getters};

use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::user::{
    policy::BadPasswordPolicy,
    value_object::{Email, HashedPassword, UserState, Username},
};

// -----Aggregate Root User------------------------------------------------------
/// The user aggregate. User is the aggregate root.
#[derive(Clone, Debug, Getters, Dissolve)]
pub struct User {
    id: Uuid,
    username: Username,
    password: HashedPassword,
    email: Email,
    state: UserState,
    bad_password_window_start: Option<OffsetDateTime>,
    bad_password_attempts: u8,
}

impl User {
    pub fn new(
        id: Uuid,
        username: Username,
        password: HashedPassword,
        email: Email,
        state: UserState,
    ) -> Self {
        Self {
            id,
            username,
            password,
            email,
            state,
            bad_password_window_start: None,
            bad_password_attempts: 0,
        }
    }

    pub fn from_repo(
        id: Uuid,
        username: String,
        password: String,
        email: String,
        state: UserState,
        bad_password_window_start: Option<OffsetDateTime>,
        bad_password_attempts: u8,
    ) -> Self {
        let username = Username::from_repo(username);
        let password = HashedPassword::from_repo(password);
        let email = Email::from_repo(email);
        Self {
            id,
            username,
            password,
            email,
            state,
            bad_password_window_start,
            bad_password_attempts,
        }
    }

    pub fn activate(&mut self) {
        self.state = UserState::Active;
    }

    pub(crate) fn lock(&mut self) {
        self.state = UserState::Locked;
    }

    pub fn unlock(&mut self) {
        self.state = UserState::Active;
        self.bad_password_window_start = None;
        self.bad_password_attempts = 0;
    }

    pub fn record_bad_password_and_attempt_lock(&mut self, policy: &BadPasswordPolicy) {
        let now = OffsetDateTime::now_utc();
        let window_duration = Duration::minutes(policy.window_minutes.into());

        let within_window = self
            .bad_password_window_start
            .is_some_and(|start| now - start <= window_duration);

        if within_window {
            // 窗口内：计数递增
            self.bad_password_attempts += 1;
        } else {
            // 窗口不存在或已过期：开启新窗口，计数从1开始
            self.bad_password_window_start = Some(now);
            self.bad_password_attempts = 1;
        }

        if self.bad_password_attempts >= policy.max_attempts {
            self.lock();
        }
    }

    pub fn change_password(&mut self, new_pwd: HashedPassword) {
        self.password = new_pwd;
    }
}
