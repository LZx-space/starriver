use crate::user::value_object::{Password, State, Username};
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

    pub fn restore(username: &str, password: &str) -> Result<Self, ApiError> {
        todo!()
    }

    // 领域能力---------------------------------------------------------------------
    pub fn change_password(&mut self, new_password: &str) -> Result<(), ApiError> {
        todo!()
    }

    pub fn add_login_event(&mut self, event: LoginEvent) {
        todo!()
    }
}

// -----entity LoginEvent---------------------------------------------------

#[derive(Debug, Serialize)]
pub struct LoginEvent {
    pub try_at: OffsetDateTime,
    pub ip: String,
    pub is_sccuess: bool,
}

pub struct SecurityEvent {}
