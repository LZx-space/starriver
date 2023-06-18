use sea_orm::sea_query::Func;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::infrastructure::security::authentication::credentials::CredentialsType;
use crate::infrastructure::security::authentication::credentials_repository::CredentialsRepository;
use crate::infrastructure::security::authentication::principal::Principal;

pub trait Authenticator {
    /// 返回支持的认证对象的类型
    fn support_credentials_type(&self) -> CredentialsType;

    /// 认证
    fn authenticate(&self, principal: &mut Principal) -> Result<(), AuthenticationError>;
}

/// 认证错误
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

impl Debug for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// -----------------------------------------
pub struct AuthenticatorDispatcher {
    authenticators: Vec<Box<dyn Authenticator>>,
}

impl AuthenticatorDispatcher {
    pub fn authenticate(&self, principal: &mut Principal) {
        for authenticator in self.authenticators.iter() {
            if authenticator.support_credentials_type()
                != principal.credentials().credentials_type()
            {
                continue;
            }
            match authenticator.authenticate(principal) {
                Ok(_) => {
                    todo!("认证成功处理")
                }
                Err(_) => {
                    todo!("认证失败处理")
                }
            }
        }
    }
}

// -----------------------------------------

pub struct UsernamePasswordCredentialsAuthenticator {
    credentials_repository: dyn CredentialsRepository<ID = String>,
}

impl Authenticator for UsernamePasswordCredentialsAuthenticator {
    fn support_credentials_type(&self) -> CredentialsType {
        CredentialsType::UsernamePassword
    }

    fn authenticate(&self, principal: &mut Principal) -> Result<(), AuthenticationError> {
        let credentials = principal.credentials();
        let content = credentials.content();
        let credentials_in_repo = self.credentials_repository.find_by_id("".to_string());
        match credentials_in_repo {
            None => Err(AuthenticationError::UsernameNotFound),
            Some(credentials) => {
                let map = credentials.content();
                principal.set_authenticated();
                Ok(())
            }
        }
    }
}
