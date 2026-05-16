use regex::{Error, Regex};
use std::sync::Arc;

#[derive(Clone)]
pub struct Patterns {
    pub email: Arc<Regex>,
    pub username: Arc<Regex>,
    pub password: Arc<Regex>,
}

impl Patterns {
    pub fn new(email_reg: &str, username_reg: &str, password_reg: &str) -> Result<Self, Error> {
        let email = Regex::new(email_reg)?;
        let username = Regex::new(username_reg)?;
        let password = Regex::new(password_reg)?;
        Ok(Self {
            email: Arc::new(email),
            username: Arc::new(username),
            password: Arc::new(password),
        })
    }
}
