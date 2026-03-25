use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserCmd {
    // #[validate(regex(path = *REGEX_USERNAME))]
    pub username: String,
    // #[validate(regex(path = *REGEX_PASSWORD))]
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
