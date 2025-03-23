use crate::user::value_object::{LoginResult, Password, State, Username};
use anyhow::Error;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

/// The user aggregate. User is the aggregate root.
#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: Username,
    #[serde(skip_serializing)]
    pub password: Password,
    pub state: State,
    pub created_at: OffsetDateTime,
    pub login_events: Vec<LoginEvent>,
}

impl User {
    pub fn new_with_username_and_password(username: &str, password: &str) -> Result<Self, Error> {
        let username = Username::new(username)?;
        let password = Password::new_by_raw_password(password)?;
        Ok(User {
            id: Uuid::now_v7(),
            username,
            password,
            state: Default::default(),
            created_at: OffsetDateTime::now_utc(),
            login_events: vec![],
        })
    }
}

// -----entity LoginEvent---------------------------------------------------

#[derive(Debug, Serialize)]
pub struct LoginEvent {
    pub login_at: OffsetDateTime,
    pub ip: String,
    pub result: LoginResult,
}
