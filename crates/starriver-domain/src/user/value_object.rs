use std::fmt::Display;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{common_error::DomainError, common_traits::PasswordEncoder};

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
    pub fn new(username: &str, regex: &Regex) -> Result<Self, DomainError> {
        if !regex.is_match(username) {
            return Err(DomainError::InvalidUsernameFormat);
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
    pub fn from_raw(
        password: &str,
        regex: &Regex,
        encoder: &impl PasswordEncoder,
    ) -> Result<Self, DomainError> {
        if !regex.is_match(password) {
            return Err(DomainError::InvalidPasswordFormat);
        }
        let hashed = encoder
            .encode(password)
            .map_err(|e| DomainError::PasswordEncoding(e.to_string()))?;
        Ok(Self(hashed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

///////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Email(pub(crate) String);

impl Email {
    pub fn new(email: &str, regex: &Regex) -> Result<Self, DomainError> {
        if !regex.is_match(email) {
            return Err(DomainError::InvalidEmailFormat);
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
