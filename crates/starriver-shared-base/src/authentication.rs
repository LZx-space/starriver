use serde::{Deserialize, Serialize};
use time::UtcDateTime;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UsernamePasswordCredentials {
    pub username: String,
    pub password: String,
}

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
    pub fn new(sub: Uuid, username: String, email: String) -> Self {
        Self {
            exp: UtcDateTime::now()
                .saturating_add(time::Duration::hours(1))
                .unix_timestamp(),
            nbf: UtcDateTime::now().unix_timestamp(),
            iat: UtcDateTime::now().unix_timestamp(),
            sub,
            username,
            email,
        }
    }
}
