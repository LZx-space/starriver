use std::sync::Arc;

use crate::service::config_service::Regex as Config;
use regex::Regex;

#[derive(Clone)]
pub struct Patterns {
    pub email: Arc<Regex>,
    pub username: Arc<Regex>,
    pub password: Arc<Regex>,
}

impl Patterns {
    pub fn new(cfg: Config) -> Self {
        Self {
            email: Regex::new(cfg.email.as_str())
                .unwrap_or_else(|e| panic!("{}", e))
                .into(),
            username: Regex::new(cfg.username.as_str())
                .unwrap_or_else(|e| panic!("{}", e))
                .into(),
            password: Regex::new(cfg.password.as_str())
                .unwrap_or_else(|e| panic!("{}", e))
                .into(),
        }
    }
}
