pub mod req {
    use std::{fmt::Display, sync::Arc};

    use serde::Deserialize;
    use starriver_identity_domain::user::specification::{PasswordSpec, UsernameSpec};
    use uuid::Uuid;
    use validator::{Validate, ValidationError};

    #[derive(Debug, Deserialize, Validate)]
    #[validate(context = UserValidateCxt)]
    pub struct UserRegisterCmd {
        #[validate(custom(function = "validate_username", use_context))]
        pub username: String,
        #[validate(custom(function = "validate_password", use_context))]
        pub password: String,
        #[validate(email)]
        pub email: String,
        #[validate(length(equal = 6))]
        pub email_code: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct UserRegisterEmailCmd {
        #[validate(email)]
        pub email: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct UserActiveEmailCmd {
        pub user_id: Uuid,
        #[validate(email)]
        pub email: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct UserActiveCmd {
        #[validate(length(equal = 6))]
        pub email_code: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    #[validate(context = UserValidateCxt)]
    pub struct ChangePasswordCmd {
        pub cur_password: String,
        #[validate(custom(function = "validate_password", use_context))]
        pub new_password: String,
        #[validate(custom(function = "validate_password", use_context))]
        pub new_password_confirm: String,
    }

    pub struct SecurityEventCmd {
        pub user_id: Uuid,
        pub event_type: SecurityEventType,
        pub payload: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum SecurityEventType {
        TryLoginWithBadPwd,
        UserUnlocked,
        PasswordChanged,
    }

    impl Display for SecurityEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SecurityEventType::TryLoginWithBadPwd => write!(f, "TryLoginWithBadPwd")?,
                SecurityEventType::UserUnlocked => write!(f, "UserUnlocked")?,
                SecurityEventType::PasswordChanged => write!(f, "PasswordChanged")?,
            }
            Ok(())
        }
    }

    //////////////////////////////////////////////////////////////////////////////////////

    #[derive(Clone)]
    pub struct UserValidateCxt {
        pub username_spec: Arc<UsernameSpec>,
        pub password_spec: Arc<PasswordSpec>,
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
}

pub mod res {
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
        pub user_name: String,
        pub event_type: String,
        pub occurred_at: OffsetDateTime,
    }
}
