use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::infrastructure::security::authentication::core::principal::Principal;
use crate::infrastructure::security::authentication::core::proof::Proof;

pub trait Authenticator {
    type Proof: Proof;

    type Principal: Principal;

    /// 认证
    fn authenticate(&self, proof: &Self::Proof) -> Result<Self::Principal, AuthenticationError> {
        // todo validate?
        self.prove(proof)
    }

    fn prove(&self, proof: &Self::Proof) -> Result<Self::Principal, AuthenticationError>;
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
