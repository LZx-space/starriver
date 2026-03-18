use crate::user::{
    specification::PasswordSpecification,
    state_object::AuthByPwdState,
    value_object::{Password, SecurityEventType, UserState, Username},
};
use starriver_infrastructure::{
    domain_state::{DomainState, State},
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
    pub login_events: Vec<SecurityEvent>,
}

impl User {
    pub fn change_password(
        &mut self,
        new_password: &str,
        spec: &PasswordSpecification,
    ) -> DomainState<Password> {
        let pwd = Password::create_password(new_password, spec);
        let pwd = match pwd {
            Ok(pwd) => pwd,
            Err(err) => {
                return DomainState::with_error(err);
            }
        };
        self.password = pwd;
        DomainState::with_state(self.password.clone())
    }

    /// 通过密码认证
    pub fn authenticate_by_password(
        &mut self,
        raw_pwd: &str,
        spec: &PasswordSpecification,
    ) -> State<AuthByPwdState, AuthenticationError> {
        // 先检查用户状态
        let state_error = match self.state {
            UserState::Active => None,
            UserState::Inactive => Some(AuthenticationError::UserInactive),
            UserState::Locked => Some(AuthenticationError::UserLocked),
            UserState::Disabled => Some(AuthenticationError::UserDisabled),
        };
        if let Some(err) = state_error {
            return State::with_error(err);
        }

        let pwd_hash_str = match from_hashed_password(self.password.hashed_password_string()) {
            Ok(hash) => hash,
            Err(e) => {
                error!(
                    "bad hashed password string in {} repository, {}",
                    self.username.as_str(),
                    e
                );
                // 数据库数据错误，不做状态变更
                return State::with_error(AuthenticationError::BadPassword);
            }
        };
        match verify_password(raw_pwd, &pwd_hash_str) {
            Ok(_) => State::with_all_none(),
            Err(_) => {
                // 密码错误，记录登录事件
                let event = SecurityEvent {
                    id: Uuid::now_v7(),
                    user_id: self.id,
                    event_type: SecurityEventType::TryLoginWithBadPwd,
                    message: "bad password".to_string(),
                    created_at: OffsetDateTime::now_utc(),
                };
                self.login_events.push(event.clone());
                // 检查是否需要锁定用户
                let locked = spec.lock_if_try_exceeded(&self.login_events);
                if locked {
                    info!("用户{}已锁定，登录密码错误太多", self.id);
                }
                // 返回状态，包含锁定状态和密码错误事件
                let state = AuthByPwdState {
                    user_id: self.id,
                    locked,
                    bad_pwd_event: Some(event),
                };
                State::with_both(state, AuthenticationError::BadPassword)
            }
        }
    }
}

// -----entity LoginEvent---------------------------------------------------

#[derive(Clone, Debug)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: SecurityEventType,
    pub message: String,
    pub created_at: OffsetDateTime,
}
