use std::sync::Arc;

use uuid::Uuid;

use crate::{
    error::DomainError,
    password_encoder::PasswordEncoder,
    user::{
        entity::User,
        value_object::{Email, EmailSpec, Password, PasswordSpec, Username, UsernameSpec},
    },
};

#[derive(Clone)]
pub struct UserFactory<PE> {
    email_spec: Arc<EmailSpec>,
    username_spec: Arc<UsernameSpec>,
    password_spec: Arc<PasswordSpec>,
    password_encoder: Arc<PE>,
}

impl<PE: PasswordEncoder> UserFactory<PE> {
    pub fn new(
        email_spec: Arc<EmailSpec>,
        username_spec: Arc<UsernameSpec>,
        password_spec: Arc<PasswordSpec>,
        password_encoder: Arc<PE>,
    ) -> Self {
        Self {
            email_spec,
            username_spec,
            password_spec,
            password_encoder,
        }
    }

    pub fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<User, DomainError> {
        let username = Username::new(username, &self.username_spec)?;
        self.password_spec.validate(password)?;
        let hashed_pwd = &self.password_encoder.encode(password)?;
        let password = Password::new(hashed_pwd)?;
        let email = Email::new(email, &self.email_spec)?;
        Ok(User::new(
            Uuid::now_v7(),
            username,
            password,
            email,
            Default::default(),
        ))
    }
}
