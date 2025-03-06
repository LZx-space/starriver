use sea_orm::prelude::async_trait::async_trait;
use std::fmt::Debug;
use thiserror::Error;

use crate::infrastructure::security::authentication::core::credential::Credential;
use crate::infrastructure::security::authentication::core::principal::Principal;

#[async_trait]
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

    #[error("username is empty")]
    UsernameEmpty,

    #[error("password not found")]
    PasswordNotFound,

    #[error("password is empty")]
    PasswordEmpty,

    #[error("bad password")]
    BadPassword,

    #[error("unknown error")]
    Unknown,
}
