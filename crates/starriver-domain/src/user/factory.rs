use starriver_infrastructure::{
    error::ApiError, security::password_encoder::PasswordEncoder, util::regex_patterns::Patterns,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::user::{
    entity::{SecurityEvent, User},
    value_object::{Email, Password, UserState, Username},
};

#[derive(Clone)]
pub struct UserFactory {
    patterns: Patterns,
}

impl UserFactory {
    pub fn new(patterns: Patterns) -> Self {
        Self { patterns }
    }

    pub fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
        password_encoder: &impl PasswordEncoder,
    ) -> Result<User, ApiError> {
        let username = Username::new(username, &self.patterns.username)?;
        let password = Password::new(password, &self.patterns.password, password_encoder)?;
        let email = Email::new(email, &self.patterns.email)?;
        Ok(User::new(
            Uuid::now_v7(),
            username,
            password,
            email,
            Default::default(),
            OffsetDateTime::now_utc(),
            vec![],
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_repo(
        &self,
        id: Uuid,
        username: &str,
        password: &str,
        email: &str,
        state: UserState,
        created_at: OffsetDateTime,
        security_events: Vec<SecurityEvent>,
    ) -> Result<User, ApiError> {
        let username = Username(username.to_string());
        let password = Password(password.to_string());
        let email = Email(email.to_string());
        Ok(User::new(
            id,
            username,
            password,
            email,
            state,
            created_at,
            security_events,
        ))
    }
}
