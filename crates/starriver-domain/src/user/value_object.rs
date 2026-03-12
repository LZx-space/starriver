use argon2::password_hash::PasswordHashString;
use serde::{Deserialize, Serialize};
use starriver_infrastructure::{
    error::error::{ApiError, Cause},
    security::password_hasher::{from_hashed_password, hash_password, verify_password},
};
use time::OffsetDateTime;

use crate::user::specification::PasswordSpecification;

#[derive(Debug, Default, Serialize)]
pub enum State {
    #[default]
    UnVerified,
    Activated,
    Disabled,
    Expired, //
}

// ----------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, ApiError> {
        if username.len() < 3 || username.len() > 20 {
            return Err(ApiError::new(
                Cause::ClientBadRequest,
                "must be less than 20 characters",
            ));
        }
        Ok(Self(username.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Password {
    hashed_string: String,
    set_at: OffsetDateTime,
}

impl Password {
    pub fn create_password(
        raw_password: &str,
        spec: &PasswordSpecification,
    ) -> Result<Self, ApiError> {
        spec.validate_new_password(raw_password)?;
        hash_password(raw_password)
            .map_err(|e| ApiError::new(Cause::ClientBadRequest, e.to_string()))
            .map(|e| Password {
                hashed_string: e.to_string(),
                set_at: OffsetDateTime::now_utc(),
            })
    }

    pub fn restore_by_hashed_pwd(
        hashed_string: &str,
        set_at: OffsetDateTime,
    ) -> Result<Self, ApiError> {
        from_hashed_password(hashed_string)
            .map_err(|e| ApiError::new(Cause::ClientBadRequest, e.to_string()))
            .map(|e| Password {
                hashed_string: e.to_string(),
                set_at,
            })
    }

    pub fn verify_password(&self, raw_pwd: &str) -> Result<(), ApiError> {
        let password_hash_string = PasswordHashString::new(&self.hashed_string)
            .map_err(|e| ApiError::new(Cause::ClientBadRequest, e.to_string()))?;
        verify_password(raw_pwd, &password_hash_string)
            .map_err(|e| ApiError::new(Cause::ClientBadRequest, e.to_string()))
    }

    pub fn hashed_password_string(&self) -> &str {
        &self.hashed_string
    }

    pub fn set_at(&self) -> OffsetDateTime {
        self.set_at
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub enum SecurityEventType {
    TryLoginWithBadPwd,
    PasswordChanged,
}
