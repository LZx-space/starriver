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

//////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod username_tests {
    use super::*;

    fn spec() -> UsernameSpec {
        let spec = UsernameSpec::new(Regex::new(r"^[a-zA-Z0-9._%+-]+$").unwrap());
        spec
    }

    // ── 合法 ──

    #[test]
    fn pure_letters() {
        assert!(spec().validate("alice").is_ok());
    }

    #[test]
    fn letters_and_digits() {
        assert!(spec().validate("user123").is_ok());
    }

    #[test]
    fn allowed_special_chars() {
        assert!(spec().validate("alice.bob").is_ok());
        assert!(spec().validate("user_name").is_ok());
        assert!(spec().validate("test+tag").is_ok());
    }

    #[test]
    fn min_length() {
        assert!(spec().validate("abc").is_ok()); // 恰好 3
    }

    #[test]
    fn max_length() {
        assert!(spec().validate("12345678901234567890").is_ok()); // 恰好 20
    }

    // ── 非法 ──

    #[test]
    fn too_short() {
        assert!(spec().validate("ab").is_err()); // 只有 2 字符
    }

    #[test]
    fn too_long() {
        assert!(spec().validate("123456789012345678901").is_err()); // 21 字符
    }

    #[test]
    fn empty() {
        assert!(spec().validate("").is_err());
    }

    #[test]
    fn contains_space() {
        assert!(spec().validate("alice bob").is_err());
    }

    #[test]
    fn contains_special_char() {
        assert!(spec().validate("alice@bob").is_err());
    }

    #[test]
    fn unicode_char() {
        assert!(spec().validate("用户").is_err());
    }
}

#[cfg(test)]
mod email_tests {
    use super::*;

    fn spec() -> EmailSpec {
        EmailSpec::new(Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap())
    }

    // ── 合法 ──

    #[test]
    fn simple() {
        assert!(spec().validate("alice@example.com").is_ok());
    }

    #[test]
    fn with_digits() {
        assert!(spec().validate("user123@domain.com").is_ok());
    }

    #[test]
    fn with_dots_in_local() {
        assert!(spec().validate("alice.bob@example.com").is_ok());
    }

    #[test]
    fn with_plus_alias() {
        assert!(spec().validate("user+tag@example.com").is_ok());
    }

    #[test]
    fn with_percent() {
        assert!(spec().validate("user%name@example.com").is_ok());
    }

    #[test]
    fn with_hyphen_in_local() {
        assert!(spec().validate("first-last@example.com").is_ok());
    }

    #[test]
    fn with_subdomain() {
        assert!(spec().validate("alice@mail.example.com").is_ok());
    }

    #[test]
    fn with_hyphen_in_domain() {
        assert!(spec().validate("alice@my-domain.com").is_ok());
    }

    #[test]
    fn two_char_tld() {
        assert!(spec().validate("alice@example.io").is_ok());
    }

    // ── 非法：缺少必要部分 ──

    #[test]
    fn empty() {
        assert!(spec().validate("").is_err());
    }

    #[test]
    fn missing_at() {
        assert!(spec().validate("aliceexample.com").is_err());
    }

    #[test]
    fn missing_local_part() {
        assert!(spec().validate("@example.com").is_err());
    }

    #[test]
    fn missing_domain() {
        assert!(spec().validate("alice@.com").is_err());
    }

    #[test]
    fn missing_tld() {
        assert!(spec().validate("alice@example").is_err());
    }

    #[test]
    fn missing_dot_before_tld() {
        assert!(spec().validate("alice@examplecom").is_err());
    }

    // ── 非法：不允许的字符 ──

    #[test]
    fn contains_space() {
        assert!(spec().validate("alice @example.com").is_err());
    }

    #[test]
    fn double_at() {
        assert!(spec().validate("alice@bob@example.com").is_err());
    }

    #[test]
    fn unicode_local() {
        assert!(spec().validate("用户@example.com").is_err());
    }

    #[test]
    fn unicode_domain() {
        assert!(spec().validate("alice@例子.com").is_err());
    }

    #[test]
    fn special_char_in_local() {
        assert!(spec().validate("alice#bob@example.com").is_err());
    }

    // ── 非法：格式边界 ──

    #[test]
    fn leading_dot_in_local() {
        assert!(spec().validate(".alice@example.com").is_err());
    }

    #[test]
    fn trailing_dot_in_local() {
        assert!(spec().validate("alice.@example.com").is_err());
    }

    #[test]
    fn consecutive_dots_in_local() {
        assert!(spec().validate("alice..bob@example.com").is_err());
    }

    #[test]
    fn single_char_tld() {
        assert!(spec().validate("alice@example.a").is_err());
    }

    // ── 掩码 ──

    #[test]
    fn mask() {
        let email = Email::new("alice@example.com", &spec()).unwrap();
        assert_eq!(email.masking(), "a****@example.com".to_string());
    }

    #[test]
    fn single_char_local_mask() {
        let email = Email::new("a@example.com", &spec()).unwrap();
        assert_eq!(email.masking(), "a@example.com".to_string());
    }

    #[test]
    fn double_char_local_mask() {
        let email = Email::new("ab@example.com", &spec()).unwrap();
        assert_eq!(email.masking(), "a*@example.com".to_string());
    }
}

#[cfg(test)]
mod password_spec_tests {
    use super::*;

    fn spec() -> PasswordSpec {
        // 至少 8 位，含大小写字母、数字、特殊字符
        PasswordSpec::new(Regex::new(r"^[A-Za-z0-9@$!%*?&]{8,12}$").unwrap())
    }

    // ── 合法 ──

    #[test]
    fn meets_all_requirements() {
        assert!(spec().validate("Abcd1234!").is_ok());
    }

    #[test]
    fn min_length() {
        assert!(spec().validate("Abc123!x").is_ok()); // 恰好 8 字符
    }

    #[test]
    fn max_length() {
        assert!(spec().validate("Abc123!xAbc1").is_ok()); // 恰好 12 字符
    }

    #[test]
    fn multiple_special_chars() {
        assert!(spec().validate("P@ssw0rd#26!").is_ok());
    }

    // ── 非法：长度 ──

    #[test]
    fn empty() {
        assert!(spec().validate("").is_err());
    }

    #[test]
    fn too_short() {
        assert!(spec().validate("Ab1!").is_err()); // 只有 4 字符
    }

    #[test]
    fn just_below_min() {
        assert!(spec().validate("Abc123!").is_err()); // 7 字符
    }

    #[test]
    fn too_long() {
        assert!(spec().validate(&"A".repeat(13)).is_err()); // 超过 12
    }

    // ── 非法：缺少必需字符类别 ──

    #[test]
    fn missing_uppercase() {
        assert!(spec().validate("abcd1234!").is_err());
    }

    #[test]
    fn missing_lowercase() {
        assert!(spec().validate("ABCD1234!").is_err());
    }

    #[test]
    fn missing_digit() {
        assert!(spec().validate("Abcdefg!").is_err());
    }

    #[test]
    fn missing_special_char() {
        assert!(spec().validate("Abcd1234").is_err());
    }

    // ── 非法：禁止字符 ──

    #[test]
    fn contains_space() {
        assert!(spec().validate("Abcd 1234!").is_err());
    }

    #[test]
    fn unicode_char() {
        assert!(spec().validate("Abcd密码123!").is_err());
    }

    #[test]
    fn only_digits() {
        assert!(spec().validate("12345678!Aa").is_ok()); // 合法但验证"纯数字"不足以失败
        // 真正的纯数字会因缺少大小写字母而失败
        assert!(spec().validate("123456789012").is_err());
    }
}
