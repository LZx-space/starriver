use crate::user::{
    specification::PasswordSpecification,
    value_object::{Password, SecurityEventType, UserState, Username},
};
use starriver_infrastructure::{
    error::ApiError,
    security::{
        authentication::core::authenticator::AuthenticationError,
        password_hasher::{from_hashed_password, verify_password},
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
    pub state: UserState,
    pub created_at: OffsetDateTime,
    pub security_events: Vec<SecurityEvent>,
}

impl User {
    pub fn change_password(
        &mut self,
        new_password: &str,
        spec: &PasswordSpecification,
    ) -> Result<(), ApiError> {
        self.password = Password::create_password(new_password, spec)?;
        Ok(())
    }

    /// 通过密码认证
    pub fn authenticate_by_password(
        &mut self,
        raw_pwd: &str,
        spec: &PasswordSpecification,
    ) -> Result<(), AuthenticationError> {
        // 先检查用户状态
        let state_error = match self.state {
            UserState::Active => None,
            UserState::Inactive => Some(AuthenticationError::UserInactive),
            UserState::Locked => Some(AuthenticationError::UserLocked),
            UserState::Disabled => Some(AuthenticationError::UserDisabled),
        };
        if let Some(err) = state_error {
            return Err(err);
        }

        from_hashed_password(self.password.hashed_password_string())
            .map_err(|e| {
                error!(
                    "bad hashed password string in {} repository, {}",
                    self.username.as_str(),
                    e
                );
                // 数据库数据错误，不做状态变更
                AuthenticationError::BadPassword
            })
            .and_then(|pwd_hash_str| {
                verify_password(raw_pwd, &pwd_hash_str).map_err(|e| {
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
                    if spec.lock_if_try_exceeded(&self.security_events) {
                        info!("用户{}已锁定，登录密码错误太多", self.id);
                        self.state = UserState::Locked;
                    }
                    AuthenticationError::BadPassword
                })
            })
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
