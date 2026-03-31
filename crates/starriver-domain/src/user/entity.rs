use crate::user::{
    policy::UserLockPolicy,
    value_object::{Email, Password, SecurityEventType, UserState, Username},
};
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
#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub username: Username,
    pub password: Password,
    pub email: Email,
    pub state: UserState,
    pub created_at: OffsetDateTime,
    pub security_events: Vec<SecurityEvent>,
}

impl User {
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
                let event = SecurityEvent {
                    id: Uuid::now_v7(),
                    user_id: self.id,
                    event_type: SecurityEventType::TryLoginWithBadPwd,
                    message: "bad password".to_string(),
                    created_at: OffsetDateTime::now_utc(),
                };
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
}

// -----entity LoginEvent---------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: SecurityEventType,
    pub message: String,
    pub created_at: OffsetDateTime,
}
