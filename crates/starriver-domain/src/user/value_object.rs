use std::fmt::Display;

use regex::Regex;
use serde::{Deserialize, Serialize};
use starriver_infrastructure::{
    error::{ApiError, Cause},
    security::password_encoder::PasswordEncoder,
};

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub enum UserState {
    /// 正常
    #[default]
    Active,
    /// 临时锁定
    Locked,
    /// 禁用/暂停
    Disabled,
}

/////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Username(pub(crate) String);

impl Username {
    pub fn new(username: &str, regex: &Regex) -> Result<Self, ApiError> {
        if !regex.is_match(username) {
            return Err(ApiError::new(
                Cause::ClientBadRequest,
                "Invalid username format".to_string(),
            ));
        }
        Ok(Self(username.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Password(pub(crate) String);

impl Password {
    pub fn new(
        password: &str,
        regex: &Regex,
        encoder: &impl PasswordEncoder,
    ) -> Result<Self, ApiError> {
        if !regex.is_match(password) {
            return Err(ApiError::new(
                Cause::ClientBadRequest,
                "Invalid password format".to_string(),
            ));
        }
        let hashed_string = encoder
            .encode(password)
            .map_err(|e| ApiError::new(Cause::ClientBadRequest, e.to_string()))?;
        Ok(Self(hashed_string))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

///////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Email(pub(crate) String);

impl Email {
    pub fn new(email: &str, regex: &Regex) -> Result<Self, ApiError> {
        if !regex.is_match(email) {
            return Err(ApiError::new(
                Cause::ClientBadRequest,
                "Invalid email format".to_string(),
            ));
        }
        Ok(Self(email.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns a masked email address, showing only the first character of the local part
    pub fn masking(&self) -> String {
        let parts: Vec<&str> = self.0.split('@').collect();
        if parts.len() != 2 {
            return self.0.clone();
        }
        let (local, domain) = (parts[0], parts[1]);
        let masked_local = format!("{}*", &local[..1]);
        format!("{}@{}", masked_local, domain)
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

///////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityEventType {
    TryLoginWithBadPwd,
    PasswordChanged,
}
