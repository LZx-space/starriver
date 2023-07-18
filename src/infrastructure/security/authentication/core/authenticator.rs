use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::infrastructure::security::authentication::core::credentials::Credentials;
use crate::infrastructure::security::authentication::core::principal::Principal;

pub trait Authenticator<T: Credentials> {
    /// 认证
    fn authenticate(&self, principal: &mut Principal<T>) -> Result<(), AuthenticationError>;
}

/// 认证错误
#[derive(Debug)]
pub enum AuthenticationError {
    /// 用户名参数未发现
    ParamUsernameNotFound,
    /// 密码参数未发现
    ParamPasswordNotFound,
    /// 用户名未发现
    UsernameNotFound,
    /// 凭证错误
    BadPassword,
}

impl Error for AuthenticationError {}

impl Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
