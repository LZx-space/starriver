use sea_orm::{DbErr, SqlErr};
use starriver_shared_base::error::RepositoryError;
use tracing::warn;

pub fn db_2_repo_error(err: DbErr) -> RepositoryError {
    warn!(error=%err, "db error");

    // 1. 先尝试 sqlx 归类的 SQL 语义错误
    if let Some(sql_err) = err.sql_err() {
        return match sql_err {
            SqlErr::UniqueConstraintViolation(_) => RepositoryError::Conflict(format_sql_err(&err)),
            SqlErr::ForeignKeyConstraintViolation(_) => {
                RepositoryError::ReferenceViolation(format_sql_err(&err))
            }
            _ => RepositoryError::Internal(Box::new(err)),
        };
    }

    // 2. 再按 DbErr 变体分流，注意每个变体的字段类型
    match err {
        // ConnectionAcquire(DbErr 的内部字段) -> Unavailable(Box<dyn Error>)
        DbErr::ConnectionAcquire(_) | DbErr::Conn(_) => RepositoryError::Unavailable(Box::new(err)),

        // RecordNotFound(String) -> NotFound(String)，直接搬运字段
        DbErr::RecordNotFound(msg) => RepositoryError::NotFound(msg),

        // RecordNotInserted / RecordNotUpdated 是单元变体，无字段
        // 它们语义更接近冲突，而非不存在
        DbErr::RecordNotInserted => {
            RepositoryError::Conflict("插入失败：记录与已有数据冲突".into())
        }
        DbErr::RecordNotUpdated => RepositoryError::NotFound("更新失败：目标记录不存在".into()),

        // Type(String) -> InvalidData { detail }
        DbErr::Type(detail) => RepositoryError::InvalidData { detail },

        // Json(String) -> InvalidData { detail }
        DbErr::Json(detail) => RepositoryError::InvalidData { detail },

        // ConvertFromU64(String) -> InvalidData { detail }
        DbErr::ConvertFromU64(detail) => RepositoryError::InvalidData {
            detail: detail.to_string(),
        },

        // TryIntoErr { source: Arc<dyn Error>, .. } -> Internal(Box<dyn Error>)
        // 需要把 Arc 转成 Box，或保留 source 链
        DbErr::TryIntoErr { source, .. } => RepositoryError::InvalidData {
            detail: source.to_string(),
        },

        // AccessDenied { permission, resource } -> PermissionDenied(String)
        DbErr::AccessDenied {
            permission,
            resource,
        } => RepositoryError::PermissionDenied(format!("{permission} @ {resource}")),

        // 统一归 Internal，保留 DbErr 作为 source
        DbErr::RbacError(_)
        | DbErr::PrimaryKeyNotSet { .. }
        | DbErr::BackendNotSupported { .. }
        | DbErr::KeyArityMismatch { .. }
        | DbErr::MutexPoisonError => RepositoryError::Internal(Box::new(err)),

        // 其余（Exec/Query/Migration/AttrNotSet/UnpackInsertId/UpdateGetPrimaryKey/Custom 等）
        _ => RepositoryError::Internal(Box::new(err)),
    }
}

/// 从 DbErr 中提取尽量简洁的结构化描述，避免整条 SQL 泄露
fn format_sql_err(err: &DbErr) -> String {
    // DbErr::Exec(RuntimeErr::SqlxError(e)) 等结构可进一步拆分，
    err.to_string()
}
