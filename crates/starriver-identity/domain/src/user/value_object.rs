use std::fmt::Display;

use crate::error::DomainError;
use crate::user::specification::{EmailSpec, UsernameSpec};

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
    pub fn new(username: &str, spec: &UsernameSpec) -> Result<Self, DomainError> {
        spec.validate(username)?;
        Ok(Self(username.to_string()))
    }

    pub(crate) fn from_repo(username: String) -> Self {
        Self(username)
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
pub struct HashedPassword(pub(crate) String);

impl HashedPassword {
    pub fn new(hashed_password: &str) -> Result<Self, DomainError> {
        if hashed_password.is_empty() {
            return Err(DomainError::InvalidPasswordHash);
        }
        // 任意现代密码哈希（argon2/bcrypt/scrypt）都远超 50 字符
        debug_assert!(
            hashed_password.len() >= 50,
            "HashedPassword 应来自密码编码器，收到非法输入"
        );
        Ok(Self(hashed_password.to_string()))
    }

    pub(crate) fn from_repo(hashed_password: String) -> Self {
        Self(hashed_password)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

///////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Email(pub(crate) String);

impl Email {
    pub fn new(email: &str, spec: &EmailSpec) -> Result<Self, DomainError> {
        spec.validate(email)?;
        Ok(Self(email.to_string()))
    }

    pub(crate) fn from_repo(email: String) -> Self {
        Self(email)
    }

    /// Returns a masked email address, showing only the first character of the local part
    pub fn masking(&self) -> String {
        let parts: Vec<&str> = self.0.split('@').collect();
        if parts.len() != 2 {
            return self.0.clone();
        }
        let (local, domain) = (parts[0], parts[1]);
        let masked_local = format!("{}{}", &local[..1], "*".repeat(local.len() - 1));
        format!("{}@{}", masked_local, domain)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ===================================================================
// 单元测试
// ===================================================================

#[cfg(test)]
mod username_integration_tests {
    use super::*;
    use crate::user::specification::UsernameSpec;
    use regex::Regex;

    /// 测试 Username::new 集成 UsernameSpec 校验
    #[test]
    fn username_new_passes_validation() {
        let spec = UsernameSpec::new(Regex::new(r"^[a-z]+$").unwrap());
        let username = Username::new("alice", &spec).unwrap();
        assert_eq!(username.as_str(), "alice");
    }

    #[test]
    fn username_new_fails_validation() {
        let spec = UsernameSpec::new(Regex::new(r"^[a-z]+$").unwrap());
        assert!(Username::new("ALICE", &spec).is_err());
    }
}

#[cfg(test)]
mod email_masking_tests {
    use super::*;
    use crate::user::specification::EmailSpec;
    use regex::Regex;

    fn dummy_spec() -> EmailSpec {
        EmailSpec::new(Regex::new(r"^.+@.+$").unwrap())
    }

    #[test]
    fn mask_normal_email() {
        let email = Email::new("alice@example.com", &dummy_spec()).unwrap();
        assert_eq!(email.masking(), "a****@example.com");
    }

    #[test]
    fn mask_single_char_local() {
        let email = Email::new("a@example.com", &dummy_spec()).unwrap();
        assert_eq!(email.masking(), "a@example.com");
    }

    #[test]
    fn mask_double_char_local() {
        let email = Email::new("ab@example.com", &dummy_spec()).unwrap();
        assert_eq!(email.masking(), "a*@example.com");
    }

    #[test]
    fn mask_no_at_sign_passthrough() {
        // 无 @ 时直接返回原文
        assert_eq!(
            Email::from_repo("no_at_sign".into()).masking(),
            "no_at_sign"
        );
    }
}

#[cfg(test)]
mod hashed_password_tests {
    use super::*;

    #[test]
    fn new_rejects_empty() {
        assert!(HashedPassword::new("").is_err());
    }

    #[test]
    #[should_panic(expected = "HashedPassword 应来自密码编码器")]
    fn new_panics_on_too_short() {
        // < 50 字符在 debug 模式下触发 panic
        let _ = HashedPassword::new("short");
    }

    #[test]
    fn new_accepts_long_hash() {
        let hash = "a".repeat(60);
        let pwd = HashedPassword::new(&hash).unwrap();
        assert_eq!(pwd.as_str(), hash);
    }
}
