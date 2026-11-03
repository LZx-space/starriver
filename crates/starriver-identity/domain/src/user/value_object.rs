use std::fmt::Display;

use derive_getters::Dissolve;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::error::DomainError;
use crate::password_encoder::PasswordEncoder;
use crate::user::policy::BadPasswordPolicy;
use crate::user::specification::{EmailSpec, PasswordSpec, UsernameSpec};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountStatus {
    /// 正常
    #[default]
    Active,
    /// 禁用
    Disabled,
    /// 删除
    Deleted,
}

#[derive(Default, Debug, Clone, Dissolve, PartialEq)]
#[dissolve(rename = "into_parts")]
pub struct LoginAttemptState {
    password_locked_until: Option<OffsetDateTime>,
    password_window_start: Option<OffsetDateTime>,
    password_attempts: u8,
}

impl LoginAttemptState {
    pub fn new(
        password_locked_until: Option<OffsetDateTime>,
        password_window_start: Option<OffsetDateTime>,
        password_attempts: u8,
    ) -> Self {
        Self {
            password_locked_until,
            password_window_start,
            password_attempts,
        }
    }

    ///// 业务方法必须只能是实体才能访问 ///////////////////////////////////////////////////////////////

    pub(crate) fn is_locked(&self) -> bool {
        self.password_locked_until
            .is_some_and(|time| time > OffsetDateTime::now_utc())
    }

    /// 是否存在待清理的密码失败状态（失败计数/计数窗口/锁定）
    pub(crate) fn has_password_attempts(&self) -> bool {
        self.password_attempts != 0
            || self.password_window_start.is_some()
            || self.password_locked_until.is_some()
    }

    pub(crate) fn reset_password_attempts(&mut self) {
        self.password_locked_until = None;
        self.password_window_start = None;
        self.password_attempts = 0;
    }

    pub(crate) fn record_password_attempts(
        &mut self,
        policy: &BadPasswordPolicy,
        now: OffsetDateTime,
    ) {
        if self.is_locked() {
            return;
        }

        let window_duration = Duration::minutes(policy.window_minutes.into());
        let within_window = self
            .password_window_start
            .is_some_and(|start| now - start <= window_duration);

        if within_window {
            // 窗口内：计数递增
            self.password_attempts += 1;
        } else {
            // 窗口不存在或已过期：开启新窗口，计数从1开始
            self.password_window_start = Some(now);
            self.password_attempts = 1;
        }

        if self.password_attempts >= policy.max_attempts {
            let lockout_duration = Duration::minutes(policy.lockout_minutes.into());
            let lockout_until = now + lockout_duration;
            self.password_locked_until = Some(lockout_until);
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str, spec: &UsernameSpec) -> Result<Self, DomainError> {
        spec.validate(username)?;
        Ok(Self(username.to_string()))
    }

    pub fn from_persisted(username: String) -> Self {
        Self(username)
    }

    pub fn into_inner(self) -> String {
        self.0
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
pub struct RawPassword(String);

impl RawPassword {
    pub fn new(raw: &str, spec: &PasswordSpec) -> Result<Self, DomainError> {
        spec.validate(raw)?;
        Ok(Self(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashedPassword(String);

impl HashedPassword {
    /// # returns
    ///  - `Ok(Self)`
    ///  - `Err(DomainError::InvalidPasswordFormat)`
    pub fn from_raw(
        raw: &RawPassword,
        encoder: &impl PasswordEncoder,
    ) -> Result<Self, DomainError> {
        let hashed_password = encoder
            .encode(&raw.0)
            .map_err(|_e| DomainError::InvalidPasswordFormat)?;
        Ok(Self(hashed_password))
    }

    pub fn from_persisted(hashed_password: String) -> Self {
        Self(hashed_password)
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    ///// 业务方法必须只能是实体才能访问 ///////////////////////////////////////////////////////////////

    /// 验证明文是否匹配
    /// # Parameters
    ///  - `raw` - 传入的明文密码，不传入`RawPassword`类型，因为老密码可能不符合当前密码规则
    ///  - `encoder` - 密码编码器
    /// # returns
    ///  - `Ok(bool)` - 如果明文密码与密码匹配，则返回 `true`，否则返回 `false
    ///  - `Err(DomainError::BadPassword)` - 如果密码不匹配，则返回 `DomainError::BadPassword`
    pub(crate) fn verify(
        &self,
        raw: &str,
        encoder: &impl PasswordEncoder,
    ) -> Result<bool, DomainError> {
        encoder
            .verify(raw, &self.0)
            .map_err(|_e| DomainError::BadPassword)
    }
}

impl Display for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

///////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    /// 与 `user.email` 列宽 `varchar(254)` 保持一致（RFC 5321 的上限）。
    pub const MAX_LEN: usize = 254;

    /// 邮箱身份的唯一规范形式：trim + lowercase。
    ///
    /// 写入（[`Email::new`]）与所有把邮箱当查询/缓存键的地方必须一致使用此函数，
    /// 保证仅大小写不同的邮箱被视为同一身份。
    pub fn normalize(raw: &str) -> String {
        raw.trim().to_lowercase()
    }

    pub fn new(email: &str, spec: &EmailSpec) -> Result<Self, DomainError> {
        let email = Self::normalize(email);

        if email.chars().count() > Self::MAX_LEN {
            return Err(DomainError::InvalidEmailFormat);
        }
        spec.validate(&email)?;
        Ok(Self(email))
    }

    pub fn from_persisted(email: String) -> Self {
        Self(email)
    }

    ///// 业务方法必须只能是实体才能访问 ///////////////////////////////////////////////////////////////

    /// Returns a masked email address, showing only the first character of the local part
    #[allow(unused)]
    pub(crate) fn masking(&self) -> String {
        let parts: Vec<&str> = self.0.split('@').collect();
        if parts.len() != 2 {
            return self.0.clone();
        }
        let (local, domain) = (parts[0], parts[1]);
        let masked_local = format!("{}{}", &local[..1], "*".repeat(local.len() - 1));
        format!("{}@{}", masked_local, domain)
    }

    pub fn into_inner(self) -> String {
        self.0
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
            Email::from_persisted("no_at_sign".into()).masking(),
            "no_at_sign"
        );
    }
}

#[cfg(test)]
mod email_normalization_tests {
    use super::*;
    use crate::user::specification::EmailSpec;
    use regex::Regex;

    fn dummy_spec() -> EmailSpec {
        EmailSpec::new(Regex::new(r"^.+@.+$").expect("valid test regex"))
    }

    #[test]
    fn normalize_trims_and_lowercases() {
        assert_eq!(
            Email::normalize("  Alice@Example.COM "),
            "alice@example.com"
        );
        assert_eq!(Email::normalize("alice@example.com"), "alice@example.com");
    }

    #[test]
    fn new_stores_normalized_email() {
        let email = Email::new("  Alice@Example.COM ", &dummy_spec()).expect("valid email");
        assert_eq!(email.as_str(), "alice@example.com");
    }

    #[test]
    fn new_accepts_email_at_column_width() {
        let email = format!("{}@example.com", "a".repeat(Email::MAX_LEN - 12));
        assert_eq!(email.chars().count(), Email::MAX_LEN);
        assert!(Email::new(&email, &dummy_spec()).is_ok());
    }

    #[test]
    fn new_rejects_email_beyond_column_width() {
        let email = format!("{}@example.com", "a".repeat(Email::MAX_LEN - 11));
        assert_eq!(email.chars().count(), Email::MAX_LEN + 1);
        assert!(matches!(
            Email::new(&email, &dummy_spec()),
            Err(DomainError::InvalidEmailFormat)
        ));
    }
}

#[cfg(test)]
mod hashed_password_tests {
    use super::*;
    use regex::Regex;

    fn dummy_spec() -> PasswordSpec {
        PasswordSpec::new(Regex::new(r"^[A-Za-z0-9@$!%*?&]{8,12}$").expect("valid test regex"))
    }

    #[test]
    fn test_returns_invalid_format() {
        let spec = dummy_spec();
        let result = RawPassword::new("short", &spec);
        assert!(result.is_err());
        let result = RawPassword::new("long1234567890", &spec);
        assert!(result.is_err());
        let result = RawPassword::new("notallow^", &spec);
        assert!(result.is_err());
        let result = RawPassword::new("abc12345", &spec);
        assert!(result.is_ok());
    }
}
