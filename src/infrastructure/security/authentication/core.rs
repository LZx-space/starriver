use actix_session::Session;
use std::env::args;
use std::ops::Deref;

/// 认证信息
pub struct Authentication {
    authenticated: bool,
    client_details: ClientDetails,
    principal: Box<dyn Principal>,
}

impl Authentication {
    /// create instance with [`principal`]and[`client_details`]
    ///
    /// [`principal`]: Principal
    /// [`client_details`]: ClientDetails
    pub fn new(principal: Box<dyn Principal>, client_details: ClientDetails) -> Self {
        Authentication {
            authenticated: false,
            client_details,
            principal,
        }
    }

    pub fn set_authenticated(&mut self, is_authenticated: bool) {
        self.authenticated = is_authenticated
    }

    pub fn get_principal(&self) -> &dyn Principal {
        self.principal.deref()
    }
}

/// 认证请求的客户端的详情，记录HTTP协议中的其它信息
pub struct ClientDetails {}

/// 认证对象
pub trait Principal {
    fn principal_type(&self) -> PrincipalType;

    fn name(&self) -> &String;

    fn credentials(&self) -> &String;
}

#[derive(PartialEq)]
pub enum PrincipalType {
    /// 用户名&密码
    UsernamePassword,
}

// -----------------------------------------------------------

pub trait Authenticator {
    type Output;

    /// 返回支持的认证对象的类型
    fn support_principal_type(&self) -> PrincipalType;

    /// 认证
    fn authenticate(
        &self,
        authentication: &mut Authentication,
    ) -> Result<Authentication, AuthenticationError>;
}

/// 认证错误
pub enum AuthenticationError {
    /// 用户名未发现
    UsernameNotFound,
}
