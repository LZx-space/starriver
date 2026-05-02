use std::sync::Arc;

use regex::Regex;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    common_error::DomainError,
    common_traits::PasswordEncoder,
    user::{
        entity::{SecurityEvent, User},
        value_object::{Email, Password, UserState, Username},
    },
};

#[derive(Clone)]
pub struct UserFactory {
    email_regex: Arc<Regex>,
    username_regex: Arc<Regex>,
    password_regex: Arc<Regex>,
}

impl UserFactory {
    pub fn new(
        email_regex: Arc<Regex>,
        username_regex: Arc<Regex>,
        password_regex: Arc<Regex>,
    ) -> Self {
        Self {
            email_regex,
            username_regex,
            password_regex,
        }
    }

    pub fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
        password_encoder: &impl PasswordEncoder,
    ) -> Result<User, DomainError> {
        let username = Username::new(username, &self.username_regex)?;
        let password = Password::from_raw(password, &self.password_regex, password_encoder)?;
        let email = Email::new(email, &self.email_regex)?;
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
    ) -> Result<User, DomainError> {
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
