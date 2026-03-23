use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserCmd {
    pub username: String,
    pub password: String,
    pub email: String,
    pub email_code: String,
}

#[derive(Debug, Deserialize)]
pub struct EmailVerifyCmd {
    pub email: String,
}
