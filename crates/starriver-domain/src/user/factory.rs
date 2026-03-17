use starriver_infrastructure::error::ApiError;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::user::{
    entity::User,
    specification::PasswordSpecification,
    value_object::{Password, Username},
};

pub struct UserFactory {}

impl UserFactory {
    pub fn create_user(
        username: &str,
        password: &str,
        password_specification: PasswordSpecification,
    ) -> Result<User, ApiError> {
        let username = Username::new(username)?;
        password_specification.validate_new_password(password)?;
        let password = Password::create_password(password, &password_specification)?;
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
