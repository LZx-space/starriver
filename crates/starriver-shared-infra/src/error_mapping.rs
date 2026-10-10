use sea_orm::DbErr;
use starriver_shared_base::error::{QueryError, RepositoryError};
use tracing::warn;

pub fn db_2_repo_error(err: DbErr) -> RepositoryError {
    warn!(error=%err, "db error");
    match err {
        DbErr::ConnectionAcquire(e) => RepositoryError::ConnectionFailed(e.to_string()),
        DbErr::TryIntoErr { from, into, source } => RepositoryError::BadData(format!(
            "TryIntoErr: from={from:?} into={into:?} source={source:?}"
        )),
        DbErr::Conn(runtime_err) => RepositoryError::ConnectionFailed(runtime_err.to_string()),
        DbErr::Exec(runtime_err) => RepositoryError::Infrastructure(runtime_err.to_string()),
        DbErr::Query(runtime_err) => RepositoryError::Infrastructure(runtime_err.to_string()),
        DbErr::ConvertFromU64(e) => RepositoryError::BadData(e.to_string()),
        DbErr::UnpackInsertId => RepositoryError::BadData("unpack insert id".to_string()),
        DbErr::UpdateGetPrimaryKey => {
            RepositoryError::BadData("update get primary key".to_string())
        }
        DbErr::RecordNotFound(e) => RepositoryError::NotFound(e),
        DbErr::AttrNotSet(e) => RepositoryError::BadData(e),
        DbErr::Custom(e) => RepositoryError::BadData(e),
        DbErr::Type(e) => RepositoryError::BadData(e),
        DbErr::Json(e) => RepositoryError::BadData(e),
        DbErr::Migration(e) => RepositoryError::BadData(e),
        DbErr::RecordNotInserted => RepositoryError::BadData("RecordNotInserted".to_string()),
        DbErr::RecordNotUpdated => RepositoryError::BadData("RecordNotUpdated".to_string()),
        DbErr::BackendNotSupported { db, ctx } => {
            RepositoryError::Infrastructure(format!("db {} not supported, {}", db, ctx))
        }
        DbErr::KeyArityMismatch { expected, received } => RepositoryError::BadData(format!(
            "key arity mismatch, expected {} received {}",
            expected, received
        )),
        DbErr::PrimaryKeyNotSet { ctx } => {
            RepositoryError::BadData(format!("primary key not set {}", ctx))
        }
        DbErr::RbacError(e) => RepositoryError::PermissionDenied(e),
        DbErr::AccessDenied {
            permission,
            resource,
        } => RepositoryError::PermissionDenied(format!(
            "access denied, permission {} resource {}",
            permission, resource
        )),
        DbErr::MutexPoisonError => RepositoryError::Infrastructure("Mutex poisoned".to_string()),
        _ => RepositoryError::Unexpected {
            message: err.to_string(),
        },
    }
}

pub fn db_2_query_error(err: DbErr) -> QueryError {
    warn!(error=%err, "db error");
    QueryError::DbError(err.to_string())
}
