use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("实体不存在：{0}")]
    NotFound(String),

    #[error("违反唯一约束：{0}")]
    Conflict(String),

    #[error("违反引用完整性：{0}")]
    ReferenceViolation(String),

    #[error("数据不满足约束：{detail}")]
    InvalidData { detail: String },

    #[error("并发修改冲突：{entity}(id={id}, expected={expected}, actual={actual})")]
    ConcurrencyConflict {
        entity: String,
        id: String,
        expected: u64,
        actual: u64,
    },

    #[error("仓储暂时不可用")]
    Unavailable(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("仓储内部错误")]
    Internal(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("权限拒绝：{0}")]
    PermissionDenied(String),
}
