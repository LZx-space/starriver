use crate::security::authentication::core::credentials::Credentials;
use crate::security::authentication::core::principal::Principal;
use std::fmt::Debug;
use thiserror::Error;

pub trait Authenticator {
    type Credentials: Credentials;

    type Principal: Principal;

    /// 认证
    fn authenticate(
        &self,
        credentials: &Self::Credentials,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send;
}

#[derive(Error, Debug, PartialEq, Eq)]
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

    /////////////////////////
    #[error("user locked")]
    UserLocked,

    #[error("user disabled")]
    UserDisabled,

    /////////////////////////
    #[error("unknown error")]
    Unknown,
}
