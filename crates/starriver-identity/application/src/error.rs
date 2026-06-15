use starriver_identity_domain::error::{DomainError, PasswordEncoderError};
use starriver_shared_base::error::RepositoryError;
use thiserror::Error;
use tracing::error;

/// 应用上下文错误，包含所有可能的错误类型
#[derive(Debug, Error)]
pub enum CtxError {
    /// 输入校验失败（用户名格式、密码强度、邮箱格式等）
    #[error("输入无效: {0}")]
    InvalidInput(String),

    /// 认证失败（密码错误、用户锁定、禁用）
    #[error("认证失败: {0}")]
    AuthenticationFailed(String),

    /// 资源不存在
    #[error("资源不存在: {0}")]
    NotFound(String),

    /// 资源冲突（重复注册等）
    #[error("资源冲突: {0}")]
    Conflict(String),

    /// 内部错误（基础设施故障，不暴露细节给客户端）
    #[error("内部服务器错误")]
    Internal,
}

#[derive(Debug, Error)]
pub enum EmailVerificationError {
    #[error("构建客户端错误：{0}")]
    BuildClientError(String),
    #[error("发送验证码错误：{0}")]
    SendCodeError(String),
}

///////////////////////////////////////////

impl From<EmailVerificationError> for CtxError {
    fn from(e: EmailVerificationError) -> Self {
        error!(error=%e, "email verification error");
        CtxError::Internal
    }
}

impl From<DomainError> for CtxError {
    fn from(e: DomainError) -> Self {
        match e {
            // 输入类 → InvalidInput
            DomainError::InvalidUsernameFormat
            | DomainError::InvalidEmailFormat
            | DomainError::InvalidPasswordFormat
            | DomainError::InvalidPasswordHash
            | DomainError::SamePassword => CtxError::InvalidInput(e.to_string()),

            // 认证类 → AuthenticationFailed
            DomainError::BadPassword | DomainError::UserLocked | DomainError::UserDisabled => {
                CtxError::AuthenticationFailed(e.to_string())
            }
            // 密码编码/验证失败是内部问题
            DomainError::PasswordEncoding(_) => {
                error!(error=%e, "password encoding error");
                CtxError::Internal
            }
        }
    }
}

impl From<RepositoryError> for CtxError {
    fn from(e: RepositoryError) -> Self {
        match e {
            RepositoryError::NotFound(_) => CtxError::NotFound(e.to_string()),
            RepositoryError::UniqueViolation { .. } => CtxError::Conflict(e.to_string()),
            // 其他基础设施错误不暴露细节
            _ => {
                error!(error=%e, "repository error");
                CtxError::Internal
            }
        }
    }
}

impl From<PasswordEncoderError> for CtxError {
    fn from(e: PasswordEncoderError) -> Self {
        error!(error=%e, "password encoder error");
        CtxError::Internal
    }
}
