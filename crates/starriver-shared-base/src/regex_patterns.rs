use regex::{Error, Regex};

pub struct Patterns {
    pub email: Regex,
    pub username: Regex,
    pub password: Regex,
}

impl Patterns {
    pub fn new(email_reg: &str, username_reg: &str, password_reg: &str) -> Result<Self, Error> {
        let email = Regex::new(email_reg)?;
        let username = Regex::new(username_reg)?;
        let password = Regex::new(password_reg)?;
        Ok(Self {
            email,
            username,
            password,
        })
    }
}
