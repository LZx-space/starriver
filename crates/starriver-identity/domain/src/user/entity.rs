use derive_getters::{Dissolve, Getters};

use time::OffsetDateTime;
use uuid::Uuid;

use crate::user::{
    policy::BadPasswordPolicy,
    value_object::{AccountStatus, Email, HashedPassword, LoginAttemptState, Username},
};

// -----Aggregate Root User------------------------------------------------------
/// The user aggregate. User is the aggregate root.
#[derive(Clone, Getters, Dissolve, PartialEq)]
#[dissolve(rename = "into_parts")]
pub struct User {
    id: Uuid,
    username: Username,
    password: HashedPassword,
    email: Email,
    status: AccountStatus,
    login_attempt_state: LoginAttemptState,
}

impl User {
    pub(crate) fn new(
        id: Uuid,
        username: Username,
        password: HashedPassword,
        email: Email,
        status: AccountStatus,
    ) -> Self {
        let login_attempt_state = LoginAttemptState::new(None, None, 0);
        Self {
            id,
            username,
            password,
            email,
            status,
            login_attempt_state,
        }
    }

    pub fn from_persisted(
        id: Uuid,
        username: Username,
        password: HashedPassword,
        email: Email,
        status: AccountStatus,
        login_attempt_state: LoginAttemptState,
    ) -> Self {
        Self {
            id,
            username,
            password,
            email,
            status,
            login_attempt_state,
        }
    }

    pub fn activate(&mut self) {
        self.status = AccountStatus::Active;
    }

    pub fn disable(&mut self) {
        self.status = AccountStatus::Disabled;
    }

    pub fn delete(&mut self) {
        self.status = AccountStatus::Deleted;
    }

    pub fn is_locked(&self) -> bool {
        self.login_attempt_state.is_locked()
    }

    /// 是否存在待清理的密码失败状态（失败计数/计数窗口/锁定）
    pub fn has_password_attempts(&self) -> bool {
        self.login_attempt_state.has_password_attempts()
    }

    pub fn reset_password_attempts(&mut self) {
        self.login_attempt_state.reset_password_attempts();
    }

    pub fn record_password_attempts(&mut self, policy: &BadPasswordPolicy, now: OffsetDateTime) {
        self.login_attempt_state
            .record_password_attempts(policy, now);
    }

    pub(crate) fn change_password(&mut self, new_pwd: HashedPassword) {
        self.password = new_pwd;
    }
}
