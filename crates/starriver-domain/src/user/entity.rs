use crate::user::specification::PasswordSpecification;
use crate::user::value_object::{LoginResult, Password, State, Username};
use anyhow::Error;
use serde::Serialize;
use starriver_infrastructure::error::error::ApiError;
use time::OffsetDateTime;
use uuid::Uuid;

// -----Aggregate Root User------------------------------------------------------
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
    // 工厂方法---------------------------------------------------------------------
    pub fn create_user(username: &str, password: &str) -> Result<Self, ApiError> {
        let username = Username::new(username)?;
        let password = Password::create_password(password)?;
        Ok(User {
            id: Uuid::now_v7(),
            username,
            password,
            state: Default::default(),
            created_at: OffsetDateTime::now_utc(),
            login_events: vec![],
        })
    }

    pub fn restore(
        username: &str,
        password: &str,
        password_specification: PasswordSpecification,
    ) -> Result<Self, Error> {
        todo!()
    }

    // 领域能力---------------------------------------------------------------------
    pub fn change_password(&mut self, new_password: &str) -> Result<(), ApiError> {
        todo!()
    }
}

// -----entity LoginEvent---------------------------------------------------

#[derive(Debug, Serialize)]
pub struct LoginEvent {
    pub login_at: OffsetDateTime,
    pub ip: String,
    pub result: LoginResult,
}
