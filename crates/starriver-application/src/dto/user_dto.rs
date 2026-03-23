use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserCmd {
    pub username: String,
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
