pub mod input {
    use std::{borrow::Cow, fmt::Display, sync::Arc};

    use serde::{Deserialize, Serialize};
    use starriver_identity_domain::user::{
        specification::{EmailSpec, PasswordSpec, UsernameSpec},
        value_object::Email,
    };
    use tracing::warn;
    use uuid::Uuid;
    use validator::{Validate, ValidationError};

    #[derive(Debug, Deserialize, Validate)]
    #[validate(context = UserValidateCxt)]
    pub struct UserRegisterCmd {
        #[validate(custom(function = "validate_username", use_context))]
        pub username: String,
        #[validate(custom(function = "validate_password", use_context))]
        pub password: String,
        #[validate(custom(function = "validate_email", use_context))]
        pub email: String,
        #[validate(length(equal = 6))]
        pub verification_code: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    #[validate(context = UserValidateCxt)]
    pub struct ChangeMyPasswordCmd {
        pub cur_password: String,
        #[validate(custom(function = "validate_password", use_context))]
        pub new_password: String,
        #[validate(custom(function = "validate_password", use_context))]
        pub new_password_confirm: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    #[validate(context = UserValidateCxt)]
    pub struct SendVerificationCodeCmd {
        #[validate(custom(function = "validate_email", use_context))]
        pub email: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    #[validate(context = UserValidateCxt)]
    pub struct ResetPasswordCmd {
        #[validate(custom(function = "validate_email", use_context))]
        pub email: String,
        #[validate(length(equal = 6))]
        pub verification_code: String,
        #[validate(custom(function = "validate_password", use_context))]
        pub new_password: String,
        #[validate(custom(function = "validate_password", use_context))]
        pub new_password_confirm: String,
    }

    /// 与 `security_event.message` 列宽 `varchar(100)` 保持一致
    const MAX_PAYLOAD_CHARS: usize = 100;

    pub struct SecurityEventCmd {
        user_id: Uuid,
        username: String,
        event_type: SecurityEventType,
        payload: Cow<'static, str>,
    }

    impl SecurityEventCmd {
        /// 构造安全事件；payload 超长时按字符截断，避免 `varchar(100)` 写库失败。
        pub fn new(
            user_id: Uuid,
            username: impl Into<String>,
            event_type: SecurityEventType,
            payload: impl Into<Cow<'static, str>>,
        ) -> Self {
            let payload = payload.into();
            let original_len = payload.chars().count();
            let payload = if original_len > MAX_PAYLOAD_CHARS {
                warn!(
                    max = MAX_PAYLOAD_CHARS,
                    original = original_len,
                    "security event payload truncated"
                );
                Cow::Owned(payload.chars().take(MAX_PAYLOAD_CHARS).collect())
            } else {
                payload
            };
            Self {
                user_id,
                username: username.into(),
                event_type,
                payload,
            }
        }

        pub fn into_parts(self) -> (Uuid, String, SecurityEventType, Cow<'static, str>) {
            (self.user_id, self.username, self.event_type, self.payload)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
    pub enum SecurityEventType {
        TryLoginWithBadPwd,
        UserUnlocked,
        PasswordChanged,
        PasswordReset,
    }

    impl Display for SecurityEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SecurityEventType::TryLoginWithBadPwd => write!(f, "TryLoginWithBadPwd")?,
                SecurityEventType::UserUnlocked => write!(f, "UserUnlocked")?,
                SecurityEventType::PasswordChanged => write!(f, "PasswordChanged")?,
                SecurityEventType::PasswordReset => write!(f, "PasswordReset")?,
            }
            Ok(())
        }
    }

    //////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone)]
    pub struct UserValidateCxt {
        pub username_spec: Arc<UsernameSpec>,
        pub password_spec: Arc<PasswordSpec>,
        pub email_spec: Arc<EmailSpec>,
    }

    fn validate_username(value: &str, ctx: &UserValidateCxt) -> Result<(), ValidationError> {
        ctx.username_spec.validate(value).map_err(|e| {
            ValidationError::new("invalid_username").with_message(e.to_string().into())
        })?;
        Ok(())
    }

    fn validate_password(value: &str, ctx: &UserValidateCxt) -> Result<(), ValidationError> {
        ctx.password_spec.validate(value).map_err(|e| {
            ValidationError::new("invalid_password").with_message(e.to_string().into())
        })?;
        Ok(())
    }

    fn validate_email(value: &str, ctx: &UserValidateCxt) -> Result<(), ValidationError> {
        let email = Email::normalize(value);
        ctx.email_spec.validate(&email).map_err(|e| {
            ValidationError::new("invalid_email").with_message(e.to_string().into())
        })?;
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn password_reset_displays_its_name() {
            assert_eq!(
                SecurityEventType::PasswordReset.to_string(),
                "PasswordReset"
            );
        }

        #[test]
        fn payload_is_truncated_to_message_column_width() {
            let cmd = SecurityEventCmd::new(
                Uuid::now_v7(),
                "test_user",
                SecurityEventType::PasswordChanged,
                "x".repeat(150),
            );
            let (_, _, _, payload) = cmd.into_parts();
            assert_eq!(payload.chars().count(), MAX_PAYLOAD_CHARS);
        }
    }
}

pub mod output {
    use super::input::SecurityEventType;
    use serde::Serialize;
    use starriver_identity_domain::user::value_object::LifeCycle;
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct UserDetail {
        pub id: Uuid,
        pub username: String,
        pub email: String,
    }

    #[derive(Serialize)]
    pub struct UserDetailDto {
        pub id: Uuid,
        pub username: String,
        pub email: String,
        pub life_cycle: LifeCycle,
        pub password_locked_until: Option<OffsetDateTime>,
        pub password_window_start: Option<OffsetDateTime>,
        pub password_attempts: i16,
    }

    #[derive(Serialize)]
    pub struct SecurityEventDto {
        pub id: Uuid,
        pub user_id: Uuid,
        pub username: String,
        pub event_type: SecurityEventType,
        pub occurred_at: OffsetDateTime,
    }
}
