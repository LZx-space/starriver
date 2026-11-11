use derive_getters::{Dissolve, Getters};

use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    error::DomainError,
    password_encoder::PasswordEncoder,
    user::{
        event::{UserSecurityEvent, UserSecurityEventKind},
        policy::BadPasswordPolicy,
        value_object::{
            AccountStatus, Email, HashedPassword, LoginAttemptState, RawPassword, Username,
        },
    },
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
    #[getter(skip)]
    events: Vec<UserSecurityEvent>,
}

impl User {
    pub fn new(username: Username, password: HashedPassword, email: Email) -> Self {
        Self {
            id: Uuid::now_v7(),
            username,
            password,
            email,
            status: Default::default(),
            login_attempt_state: Default::default(),
            events: Default::default(),
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
            events: Default::default(),
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

    /// # Parameters
    ///
    /// - `user`: 用户
    /// - `raw_password`: 原始密码，不用值类型，避免动态修改规则老密码无法传入
    ///
    /// # Returns
    ///
    /// - `Ok(())` - 密码匹配，用户认证成功
    /// - `Err(DomainError::UserLocked)` - 用户被锁定
    /// - `Err(DomainError::UserDisabled)` - 用户被禁用
    /// - `Err(DomainError::UserDeleted)` - 用户已删除
    /// - `Err(DomainError::BadPassword)` - 密码不匹配，并记录失败次数（达上限锁定）
    /// - `Err(DomainError::PasswordVerification)` - 密码校验失败（编码器故障）
    pub fn authenticate(
        &mut self,
        raw_password: &str,
        password_policy: &BadPasswordPolicy,
        password_encoder: &impl PasswordEncoder,
    ) -> Result<(), DomainError> {
        let now = OffsetDateTime::now_utc();
        self.ensure_authenticatable(now)?;
        let matches = self.password.verify(raw_password, password_encoder)?;
        if !matches {
            self.handle_bad_password_event(password_policy, now);
            return Err(DomainError::BadPassword);
        }
        self.login_attempt_state.reset_password_attempts();

        let event = self.create_event(UserSecurityEventKind::LoginSucceeded, now, None);
        self.events.push(event);
        Ok(())
    }

    /// # params
    /// - `cur_pwd` the current password of the user
    /// - `new_pwd` the new password that the user wants to use
    ///
    /// # returns
    /// - `Ok(())` - password changed successfully
    /// - `Err(DomainError::UserDisabled)` - user is disabled or deleted
    /// - `Err(DomainError::UserLocked)` - user is locked
    /// - `Err(DomainError::SamePassword)` - new password is same as current password
    /// - `Err(DomainError::BadPassword)` - incorrect current password
    /// - `Err(DomainError::InvalidPasswordFormat)` - invalid password format
    pub fn change_my_password(
        &mut self,
        cur_pwd: &str,
        new_pwd: &RawPassword,
        password_policy: &BadPasswordPolicy,
        password_encoder: &impl PasswordEncoder,
    ) -> Result<(), DomainError> {
        let now = OffsetDateTime::now_utc();
        self.ensure_authenticatable(now)?;

        if cur_pwd == new_pwd.as_str() {
            return Err(DomainError::SamePassword);
        };

        let matches = self.password.verify(cur_pwd, password_encoder)?;
        if !matches {
            self.handle_bad_password_event(password_policy, now);
            return Err(DomainError::BadPassword);
        }

        let password = HashedPassword::from_raw(new_pwd, password_encoder)?;
        self.password = password;
        self.login_attempt_state.reset_password_attempts();

        let event = self.create_event(UserSecurityEventKind::PasswordChanged, now, None);
        self.events.push(event);
        Ok(())
    }

    pub fn reset_password(
        &mut self,
        new_pwd: &RawPassword,
        password_encoder: &impl PasswordEncoder,
    ) -> Result<(), DomainError> {
        let password = HashedPassword::from_raw(new_pwd, password_encoder)?;
        self.password = password;
        self.login_attempt_state.reset_password_attempts();

        let event = self.create_event(
            UserSecurityEventKind::PasswordReset,
            OffsetDateTime::now_utc(),
            None,
        );
        self.events.push(event);
        Ok(())
    }

    pub fn take_events(&mut self) -> Vec<UserSecurityEvent> {
        std::mem::take(&mut self.events)
    }
    /////////////////////////////////////////////////////////////////////////////////////

    fn ensure_authenticatable(&self, now: OffsetDateTime) -> Result<(), UnAuthenticatableKind> {
        match self.status {
            AccountStatus::Active => {}
            AccountStatus::Disabled => {
                return Err(UnAuthenticatableKind::Disabled);
            }
            AccountStatus::Deleted => {
                return Err(UnAuthenticatableKind::Deleted);
            }
        };

        if self.is_locked_at(now) {
            return Err(UnAuthenticatableKind::Locked);
        }
        Ok(())
    }

    fn is_locked_at(&self, now: OffsetDateTime) -> bool {
        self.login_attempt_state.is_locked_at(now)
    }

    fn handle_bad_password_event(
        &mut self,
        password_policy: &BadPasswordPolicy,
        now: OffsetDateTime,
    ) {
        let event = self.create_event(UserSecurityEventKind::LoginWithBadPassword, now, None);
        self.events.push(event);

        let was_locked = self.is_locked_at(now);
        self.login_attempt_state
            .record_password_attempts(password_policy, now);
        if !was_locked && self.is_locked_at(now) {
            let event = self.create_event(UserSecurityEventKind::UserLocked, now, None);
            self.events.push(event);
        }
    }

    fn create_event(
        &self,
        kind: UserSecurityEventKind,
        occurred_at: OffsetDateTime,
        payload: Option<serde_json::Value>,
    ) -> UserSecurityEvent {
        UserSecurityEvent::new(
            self.id,
            self.username.as_str().to_string(),
            kind,
            occurred_at,
            payload,
        )
    }
}
/////////////////////////////////////////////////////////////////////////////////////

pub enum UnAuthenticatableKind {
    Disabled,
    Deleted,
    Locked,
}
