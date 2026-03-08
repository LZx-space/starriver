use crate::user::{
    specification::PasswordSpecification,
    value_object::{Password, State, Username},
};
use serde::Serialize;
use starriver_infrastructure::{
    error::error::ApiError,
    security::authentication::{
        core::authenticator::AuthenticationError,
        password_hasher::{from_hashed_password, verify_password},
    },
};
use time::OffsetDateTime;
use tracing::error;
use uuid::Uuid;

// -----Aggregate Root User------------------------------------------------------
/// The user aggregate. User is the aggregate root.
#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: Username,
    #[serde(skip_serializing)]
    pub password: Password,
    pub state: State,
    pub created_at: OffsetDateTime,
    pub login_events: Vec<LoginEvent>,
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
    pub fn authenticate_by_password(&mut self, raw_pwd: &str) -> Result<(), AuthenticationError> {
        from_hashed_password(self.password.hashed_password_string())
            .map_err(|e| {
                error!(
                    "bad hashed password string in {} repository, {}",
                    self.username.as_str(),
                    e
                );
                AuthenticationError::BadPassword
            })
            .and_then(|pwd_hash_str| {
                verify_password(raw_pwd, &pwd_hash_str).map_err(|e| {
                    error!(
                        "verify {} hashed password error: {}",
                        self.username.as_str(),
                        e
                    );
                    AuthenticationError::BadPassword
                })
            })
    }
}

// -----entity LoginEvent---------------------------------------------------

#[derive(Debug, Serialize)]
pub struct LoginEvent {
    pub try_at: OffsetDateTime,
    pub ip: String,
    pub is_sccuess: bool,
}

pub struct SecurityEvent {}
