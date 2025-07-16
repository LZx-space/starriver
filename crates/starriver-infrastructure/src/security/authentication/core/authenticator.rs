use crate::security::authentication::core::credential::Credential;
use crate::security::authentication::core::principal::Principal;
use std::fmt::Debug;
use thiserror::Error;

pub trait Authenticator: Send {
    type Credential: Credential;

    type Principal: Principal;

    /// 认证
    fn authenticate(
        &self,
        credential: &Self::Credential,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send;
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
