use regex::Regex;

use crate::error::DomainError;

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

/// 原始密码格式规范，面向用户输入的明文密码。
/// 仅被 UserFactory 在校验原始密码时消费。
#[derive(Clone)]
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
// 单元测试：仅验证 *Spec 委托机制，不绑定具体业务正则
// 具体正则的业务规则测试见 starriver-shared-base/src/regex_patterns.rs
// ===================================================================

#[cfg(test)]
mod username_spec_tests {
    use super::*;

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
}
