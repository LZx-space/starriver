use serde::{Deserialize, Serialize};
use time::{Duration, UtcDateTime};
use uuid::Uuid;

use crate::middleware::authentication::core::credentials::Credentials;

#[derive(Clone, Debug)]
pub struct UsernamePasswordCredentials {
    pub username: String,
    pub password: String,
}

impl Credentials for UsernamePasswordCredentials {}

/////////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Serialize)]
pub struct PrincipalClaims {
    exp: i64,      // Expiration time (as UTC timestamp)
    nbf: i64,      // Not Before (as UTC timestamp)
    iat: i64,      // Issued at (as UTC timestamp)
    pub sub: Uuid, // Subject (whom token refers to)
    pub username: String,
    pub email: String,
}

impl PrincipalClaims {
    pub fn new(exp: Duration, sub: Uuid, username: String, email: String) -> Self {
        let now = UtcDateTime::now();
        Self {
            exp: now.saturating_add(exp).unix_timestamp(),
            nbf: now.unix_timestamp(),
            iat: now.unix_timestamp(),
            sub,
            username,
            email,
        }
    }
}
