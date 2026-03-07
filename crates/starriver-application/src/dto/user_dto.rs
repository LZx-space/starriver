use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserCmd {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserDetail {
    pub username: String,
    pub password: String,
}

pub struct UsernamePasswordAuthentication {
    pub username: String,
    pub password: String,
}

#[derive(FromQueryResult)]
pub struct SecurityUser {
    pub username: String,
    pub password: String,
}
