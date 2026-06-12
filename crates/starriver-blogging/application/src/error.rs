use sea_orm::DbErr;
use starriver_blogging_domain::shared_error::DomainError;
use starriver_shared_base::{
    error::{QueryError, RepositoryError},
    io::{AsyncReaderError, AsyncWriterError},
};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum CtxError {
    /// 输入校验失败（用户名格式、密码强度、邮箱格式等）
    #[error("输入无效: {0}")]
    InvalidInput(String),

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

///////////////////////////////////////////

impl From<DomainError> for CtxError {
    fn from(e: DomainError) -> Self {
        match e {
            // 输入类 → InvalidInput
            DomainError::PostCategoryIsNone
            | DomainError::PostContentIsEmpty
            | DomainError::PostTitleIsEmpty
            | DomainError::PostTitleTooLong(_)
            | DomainError::PostCategoryTooLong(_)
            | DomainError::PostContentTooLong(_)
            | DomainError::AttachmentExtensionInvalid(_)
            | DomainError::AttachmentFileSizeInvalid(_) => CtxError::InvalidInput(e.to_string()),
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
                error!(error=%e, "database error");
                CtxError::Internal
            }
        }
    }
}

impl From<QueryError> for CtxError {
    fn from(e: QueryError) -> Self {
        error!(error=%e, "database error");
        CtxError::Internal
    }
}

impl From<AsyncReaderError> for CtxError {
    fn from(value: AsyncReaderError) -> Self {
        error!(error=%value, "IO error");
        CtxError::Internal
    }
}

impl From<AsyncWriterError> for CtxError {
    fn from(value: AsyncWriterError) -> Self {
        error!(error=%value, "IO error");
        CtxError::Internal
    }
}

pub fn db_2_ctx_error(e: DbErr) -> CtxError {
    error!(error=%e, "database error");
    CtxError::Internal
}
