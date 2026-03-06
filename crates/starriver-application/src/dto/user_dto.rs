use sea_orm::FromQueryResult;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserCmd {
    pub username: String,
    pub password: String,
}

#[derive(FromQueryResult)]
pub struct SecurityUser {
    pub username: String,
    pub password: String,
}
