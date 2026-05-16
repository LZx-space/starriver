use std::sync::Arc;

use regex::Regex;
use uuid::Uuid;

use crate::{
    password_encoder::PasswordEncoder,
    shared_error::DomainError,
    user::{
        entity::User,
        value_object::{Email, Password, Username},
    },
};

#[derive(Clone)]
pub struct UserFactory<PE> {
    email_regex: Arc<Regex>,
    username_regex: Arc<Regex>,
    password_regex: Arc<Regex>,
    password_encoder: Arc<PE>,
}

impl<PE: PasswordEncoder> UserFactory<PE> {
    pub fn new(
        email_regex: Arc<Regex>,
        username_regex: Arc<Regex>,
        password_regex: Arc<Regex>,
        password_encoder: Arc<PE>,
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
