use serde::Serialize;

use crate::security::authentication::core::{
    authenticator::AuthenticationError, principal::Principal,
};

/// 处理 Principal
///
/// * 认证成功时：将已认证主体生成关联令牌。如JWT、SessionId
/// * 用户提交时：认证令牌并将其转为被认证主体

pub trait PrincipalHandler {
    type Principal: Principal;

    type Token: Serialize;

    /// 生成一个令牌，其能通过Self::authenticate_token解析回Principal
    fn generate_token(
        &self,
        principal: &Self::Principal,
    ) -> impl Future<Output = Result<Self::Token, AuthenticationError>> + Send;

    /// 认证令牌以获取 Principal
    fn authenticate_token(
        &self,
        token: &Self::Token,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send;
}
