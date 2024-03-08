use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::infrastructure::security::authentication::core::credential::Credential;
use crate::infrastructure::security::authentication::core::principal::Principal;

pub trait Authenticator {
    type Credential: Credential;

    type Principal: Principal;

    /// 认证
    fn authenticate(
        &self,
        credential: &Self::Credential,
    ) -> Result<Self::Principal, AuthenticationError> {
        // todo validate?
        self.do_authenticate(credential)
    }

    fn do_authenticate(
        &self,
        credential: &Self::Credential,
    ) -> Result<Self::Principal, AuthenticationError>;
}

/// 认证错误
#[derive(Debug)]
pub enum AuthenticationError {
    /// 用户名未发现
    UsernameNotFound,
    /// 凭证错误
    BadPassword,
}

impl Error for AuthenticationError {}

impl Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // todo 打印具体的异常名称
        write!(f, "authenticate error")
    }
}
