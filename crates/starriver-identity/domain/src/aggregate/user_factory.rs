use std::sync::Arc;

use regex::Regex;
use uuid::Uuid;

use crate::{
    aggregate::{
        user::User,
        user_value_object::{Email, Password, Username},
    },
    common::{error::DomainError, traits::PasswordEncoder},
};

#[derive(Clone)]
pub struct UserFactory<PE: PasswordEncoder> {
    email_regex: Arc<Regex>,
    username_regex: Arc<Regex>,
    password_regex: Arc<Regex>,
    password_encoder: PE,
}

impl<PE: PasswordEncoder> UserFactory<PE> {
    pub fn new(
        email_regex: Arc<Regex>,
        username_regex: Arc<Regex>,
        password_regex: Arc<Regex>,
        password_encoder: PE,
    ) -> Self {
        Self {
            email_regex,
            username_regex,
            password_regex,
            password_encoder,
        }
    }

    pub fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<User, DomainError> {
        let username = Username::new(username, &self.username_regex)?;
        if !self.password_regex.is_match(password) {
            return Err(DomainError::InvalidPasswordFormat);
        }
        let hashed_pwd = &self
            .password_encoder
            .encode(password)
            .map_err(|e| DomainError::PasswordEncoding(e.to_string()))?;
        let password = Password::new(hashed_pwd)?;
        let email = Email::new(email, &self.email_regex)?;
        Ok(User::new(
            Uuid::now_v7(),
            username,
            password,
            email,
            Default::default(),
        ))
    }
}
