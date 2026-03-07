use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserCmd {
    pub username: String,
    pub password: String,
}
