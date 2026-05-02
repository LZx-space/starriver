use std::sync::Arc;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AuthConfig {
    pub jws_secret: Arc<String>,
}

impl AuthConfig {
    pub fn jws_secret_as_ref(&self) -> &[u8] {
        self.jws_secret.as_bytes()
    }
}
