use crate::user::{
    policy::UserLockPolicy,
    value_object::{Email, Password, SecurityEventType, UserState, Username},
};
use derive_getters::{Dissolve, Getters};
use regex::Regex;
use starriver_infrastructure::{
    error::ApiError,
    security::{
        authentication::core::authenticator::AuthenticationError, password_encoder::PasswordEncoder,
    },
};
use time::OffsetDateTime;
use tracing::{error, info};
use uuid::Uuid;

// -----Aggregate Root User------------------------------------------------------
/// The user aggregate. User is the aggregate root.
#[derive(Debug, Getters, Dissolve)]
pub struct User {
    id: Uuid,
    username: Username,
    password: Password,
    email: Email,
    state: UserState,
    created_at: OffsetDateTime,
    #[getter(skip)]
    security_events: Vec<SecurityEvent>,
}

impl User {
    pub(super) fn new(
        id: Uuid,
        username: Username,
        password: Password,
        email: Email,
        state: UserState,
        created_at: OffsetDateTime,
        security_events: Vec<SecurityEvent>,
    ) -> Self {
        Self {
            id,
            username,
            password,
            email,
            state,
            created_at,
            security_events,
        }
    }

    pub fn encode_password(
        &self,
        raw_password: &str,
        regex: &Regex,
        encoder: &impl PasswordEncoder,
    ) -> Result<Password, ApiError> {
        let pwd = Password::new(raw_password, regex, encoder)?;
        Ok(pwd)
    }

    /// 通过密码认证
    /// # 返回值
    /// * `Ok(())` - 认证成功
    /// * `Err(AuthenticationError)` - 认证失败
    ///     * `AuthenticationError::UserLocked` - 用户已锁定
    ///     * `AuthenticationError::UserDisabled` - 用户已禁用
    ///     * `AuthenticationError::BadPassword` - 密码不匹配
    pub fn authenticate_by_password(
        &mut self,
        raw_pwd: &str,
        policy: &UserLockPolicy,
        encoder: &impl PasswordEncoder,
    ) -> Result<(), AuthenticationError> {
        // 先检查用户状态
        match self.state {
            UserState::Active => {}
            UserState::Locked => return Err(AuthenticationError::UserLocked),
            UserState::Disabled => return Err(AuthenticationError::UserDisabled),
        };

        encoder
            .verify(raw_pwd, self.password.as_str())
            .map(|_| Ok(()))
            .map_err(|e| {
                error!(
                    "verify {} hashed password error: {}",
                    self.username.as_str(),
                    e
                );
                // 密码错误，记录登录事件
                let event = SecurityEvent::new(
                    self.id,
                    SecurityEventType::TryLoginWithBadPwd,
                    "bad password",
                );
                self.security_events.push(event);
                // 检查是否需要锁定用户
                if policy.should_lock(&self.security_events) {
                    info!("user[{}] locked，bad password too many times", self.id);
                    self.state = UserState::Locked;
                }
                AuthenticationError::BadPassword
            })?
    }

    pub fn activate(&mut self) {
        self.state = UserState::Active;
    }

    pub fn security_events(&mut self) -> &mut Vec<SecurityEvent> {
        &mut self.security_events
    }
}

// -----entity LoginEvent---------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Getters, Dissolve)]
pub struct SecurityEvent {
    id: Uuid,
    user_id: Uuid,
    event_type: SecurityEventType,
    message: String,
    created_at: OffsetDateTime,
}

impl SecurityEvent {
    pub(super) fn new(user_id: Uuid, event_type: SecurityEventType, message: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id,
            event_type,
            message: message.into(),
            created_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn from_repo(
        id: Uuid,
        user_id: Uuid,
        event_type: SecurityEventType,
        message: String,
        created_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            user_id,
            event_type,
            message,
            created_at,
        }
    }
}
