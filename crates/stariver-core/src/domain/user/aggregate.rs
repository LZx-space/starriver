use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub state: State,
    pub created_at: OffsetDateTime,
    pub login_records: Vec<LoginRecord>,
}

impl User {
    pub fn new_with_username_and_password(username: &str, password: &str) -> User {
        User {
            id: Uuid::now_v7(),
            username: username.to_string(),
            password: password.to_string(),
            state: Default::default(),
            created_at: OffsetDateTime::now_utc(),
            login_records: vec![],
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub enum State {
    #[default]
    NotVerified,
    Normal,
    Forbidden,
}

#[derive(Debug, Serialize)]
pub struct LoginRecord {
    pub login_at: OffsetDateTime,
    pub success: bool,
    pub ip: String,
}
