use serde::Serialize;

use crate::security::authentication::core::{
    authenticator::AuthenticationError, principal::Principal,
};

/// 处理 Principal
///
/// * 认证成功时：将 Principal 转为标识
/// * 用户提交时：将标识转为 Principal

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
