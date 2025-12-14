use crate::user::entity::LoginEvent;
use anyhow::Error;
use argon2::password_hash::PasswordHashString;
use serde::{Deserialize, Serialize};
use starriver_infrastructure::security::authentication::password_hasher::{
    from_hashed_password, hash_password, verify_password,
};
use time::OffsetDateTime;

#[derive(Debug, Default, Serialize)]
pub enum State {
    #[default]
    UnVerified,
    Activated,
    Disabled,
    Expired, //
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, Error> {
        if username.len() < 3 || username.len() > 20 {
            return Err(Error::msg("must be less than 20 characters"));
        }
        Ok(Self(username.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Clone, Debug, Deserialize)]
pub struct Password {
    #[serde(skip_serializing)]
    hashed_string: String,
    set_at: OffsetDateTime,
}

impl Password {
    pub fn create_password(raw_password: &str) -> Result<Self, Error> {
        hash_password(raw_password)
            .map_err(|e| Error::msg(e.to_string()))
            .map(|e| Password {
                hashed_string: e.to_string(),
                set_at: OffsetDateTime::now_utc(),
            })
    }

    pub fn restore_by_hashed_pwd(
        hashed_string: &str,
        set_at: OffsetDateTime,
    ) -> Result<Self, Error> {
        from_hashed_password(hashed_string)
            .map_err(|e| Error::msg(e.to_string()))
            .map(|e| Password {
                hashed_string: e.to_string(),
                set_at,
            })
    }

    pub fn verify_password(&self, input: &str) -> Result<(), Error> {
        let password_hash_string =
            PasswordHashString::new(&self.hashed_string).map_err(|e| Error::msg(e.to_string()))?;
        verify_password(input, &password_hash_string)
            .map(|_| ())
            .map_err(|e| Error::msg(e))
    }

    pub fn hashed_password_string(&self) -> &str {
        &self.hashed_string
    }

    pub fn set_at(&self) -> OffsetDateTime {
        self.set_at
    }
}

// ----- Login Event --------------------------------------

pub struct RecentLoginEvents {
    pub login_events: Vec<LoginEvent>,
}

#[derive(Debug, Serialize)]
pub enum LoginResult {
    Success,
    Failure(String),
}
