use std::fmt::Display;

use regex::Regex;

use crate::error::DomainError;

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

pub struct UsernameSpec(Regex);

impl UsernameSpec {
    pub fn new(regex: Regex) -> Self {
        Self(regex)
    }

    pub fn validate(&self, username: &str) -> Result<(), DomainError> {
        if !self.0.is_match(username) {
            return Err(DomainError::InvalidUsernameFormat);
        }
        Ok(())
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

    pub(crate) fn from_repo(hashed_password: String) -> Self {
        Self(hashed_password)
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

pub struct PasswordSpec(Regex);

impl PasswordSpec {
    pub fn new(regex: Regex) -> Self {
        Self(regex)
    }

    pub fn validate(&self, password: &str) -> Result<(), DomainError> {
        if !self.0.is_match(password) {
            return Err(DomainError::InvalidPasswordFormat);
        }
        Ok(())
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

pub struct EmailSpec(Regex);

impl EmailSpec {
    pub fn new(regex: Regex) -> Self {
        Self(regex)
    }

    pub fn validate(&self, email: &str) -> Result<(), DomainError> {
        if !self.0.is_match(email) {
            return Err(DomainError::InvalidEmailFormat);
        }
        Ok(())
    }
}

// ===================================================================
// 单元测试：仅验证 *Spec 封装机制，不绑定具体业务正则
// 具体正则的业务规则测试见 starriver-shared-base/src/regex_patterns.rs
// ===================================================================

#[cfg(test)]
mod username_spec_tests {
    use super::*;

    /// 测试 UsernameSpec 正确委托给 Regex
    #[test]
    fn validate_returns_ok_when_regex_matches() {
        let spec = UsernameSpec::new(Regex::new(r"^alice$").unwrap());
        assert!(spec.validate("alice").is_ok());
    }

    #[test]
    fn validate_returns_err_when_regex_mismatches() {
        let spec = UsernameSpec::new(Regex::new(r"^alice$").unwrap());
        assert!(spec.validate("bob").is_err());
    }

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
mod email_spec_tests {
    use super::*;

    #[test]
    fn validate_returns_ok_when_regex_matches() {
        let spec = EmailSpec::new(Regex::new(r"^x@y\.z$").unwrap());
        assert!(spec.validate("x@y.z").is_ok());
    }

    #[test]
    fn validate_returns_err_when_regex_mismatches() {
        let spec = EmailSpec::new(Regex::new(r"^x@y\.z$").unwrap());
        assert!(spec.validate("no_at_sign").is_err());
    }
}

#[cfg(test)]
mod email_masking_tests {
    use super::*;

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
mod password_spec_tests {
    use super::*;

    #[test]
    fn validate_returns_ok_when_regex_matches() {
        let spec = PasswordSpec::new(Regex::new(r"^secret123$").unwrap());
        assert!(spec.validate("secret123").is_ok());
    }

    #[test]
    fn validate_returns_err_when_regex_mismatches() {
        let spec = PasswordSpec::new(Regex::new(r"^secret123$").unwrap());
        assert!(spec.validate("wrong").is_err());
    }

    /// 测试 Password::new 的独立校验逻辑（非 Spec 部分）
    #[test]
    fn password_new_rejects_empty() {
        assert!(Password::new("").is_err());
    }

    #[test]
    fn password_new_rejects_too_short() {
        // < 50 字符
        assert!(Password::new("short").is_err());
    }

    #[test]
    fn password_new_accepts_long_hash() {
        let hash = "a".repeat(60);
        let pwd = Password::new(&hash).unwrap();
        assert_eq!(pwd.as_str(), hash);
    }
}
