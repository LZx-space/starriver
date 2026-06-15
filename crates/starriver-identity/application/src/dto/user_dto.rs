pub mod req {
    use std::sync::Arc;

    use serde::Deserialize;
    use starriver_identity_domain::user::specification::{PasswordSpec, UsernameSpec};
    use uuid::Uuid;
    use validator::{Validate, ValidationError};

    #[derive(Debug, Deserialize, Validate)]
    #[validate(context = UserValidateCxt)]
    pub struct UserCmd {
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
    pub struct EmailVerifyCmd {
        #[validate(email)]
        pub email: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct EmailActiveCmd {
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
        pub cur_password_confirm: String,
        #[validate(custom(function = "validate_password", use_context))]
        pub new_password: String,
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
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct UserDetail {
        pub id: Uuid,
        pub username: String,
        pub email: String,
    }
}
