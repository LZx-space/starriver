use std::fmt::Display;

use regex::Regex;

use crate::shared_error::DomainError;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Password(pub(crate) String);

impl Password {
    pub fn new(hashed_password: &str) -> Result<Self, DomainError> {
        if hashed_password.is_empty() {
            return Err(DomainError::InvalidPasswordFormat);
        }
        // 任意现代密码哈希（argon2/bcrypt/scrypt）都远超 50 字符
        if hashed_password.len() < 50 {
            return Err(DomainError::InvalidPasswordFormat);
        }
        Ok(Self(hashed_password.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
