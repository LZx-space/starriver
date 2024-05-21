use std::fmt::{Debug, Display};

use thiserror::Error;

use crate::infrastructure::security::authentication::core::credential::Credential;
use crate::infrastructure::security::authentication::core::principal::Principal;

#[trait_variant::make(HttpService: Send)]
pub trait Authenticator {
    type Credential: Credential;

    type Principal: Principal;

    /// 认证
    async fn authenticate(
        &self,
        credential: &Self::Credential,
    ) -> Result<Self::Principal, AuthenticationError>;
}

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("username not found")]
    UsernameNotFound,

    #[error("bad password")]
    BadPassword,

    #[error("unknown error")]
    Unknown,
}
