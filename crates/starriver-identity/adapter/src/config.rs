use std::sync::Arc;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdentityConfig {
    pub regexes: Regexes,
    pub bad_password: BadPasssword,
    pub auth_cfg: AuthConfig,
}

#[derive(Deserialize)]
pub struct Regexes {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct BadPasssword {
    pub window_minutes: u64,
    pub max_attempts: u64,
}

#[derive(Clone, Deserialize)]
pub struct AuthConfig {
    pub jws_secret: Arc<String>,
}

impl AuthConfig {
    pub fn jws_secret_as_ref(&self) -> &[u8] {
        self.jws_secret.as_bytes()
    }
}
