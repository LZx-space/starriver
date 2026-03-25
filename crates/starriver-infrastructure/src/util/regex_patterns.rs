use crate::service::config_service::Regex as Config;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Patterns {
    pub email: Regex,
    pub username: Regex,
    pub password: Regex,
}

impl Patterns {
    pub fn new(cfg: Config) -> Self {
        Self {
            email: Regex::new(cfg.email.as_str()).unwrap_or_else(|e| panic!("{}", e)),
            username: Regex::new(cfg.username.as_str()).unwrap_or_else(|e| panic!("{}", e)),
            password: Regex::new(cfg.password.as_str()).unwrap_or_else(|e| panic!("{}", e)),
        }
    }
}
