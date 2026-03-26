use serde::Deserialize;
use starriver_infrastructure::util::regex_patterns::Patterns;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
#[validate(context = Patterns)]
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

//////////////////////////////////////////////////////////////////////////////////////

fn validate_username(value: &str, context: &Patterns) -> Result<(), ValidationError> {
    if !context.username.is_match(value) {
        return Err(ValidationError::new("username does not match pattern"));
    }
    Ok(())
}

fn validate_password(value: &str, context: &Patterns) -> Result<(), ValidationError> {
    if !context.password.is_match(value) {
        return Err(ValidationError::new("password does not match pattern"));
    }
    Ok(())
}
