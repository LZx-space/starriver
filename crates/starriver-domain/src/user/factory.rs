use starriver_infrastructure::error::ApiError;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::user::{
    entity::User,
    specification::PasswordSpecification,
    value_object::{Email, Password, Username},
};

pub struct UserFactory {}

impl UserFactory {
    pub fn create_user(
        username: &str,
        password: &str,
        email: &str,
        password_specification: PasswordSpecification,
    ) -> Result<User, ApiError> {
        let username = Username::new(username)?;
        password_specification.validate_new_password(password)?;
        let password = Password::create_password(password, &password_specification)?;
        let email = Email::new(email)?;
        Ok(User {
            id: Uuid::now_v7(),
            username,
            password,
            email,
            state: Default::default(),
            created_at: OffsetDateTime::now_utc(),
            security_events: vec![],
        })
    }
}
