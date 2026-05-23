use sea_orm::DbErr;
use starriver_shared_base::error::{QueryError, RepositoryError};
use tracing::warn;

pub fn db_2_repo_error(err: DbErr) -> RepositoryError {
    warn!(error=%err, "db error");
    match err {
        DbErr::ConnectionAcquire(conn_acquire_err) => {
            RepositoryError::ConnectionFailed(conn_acquire_err.to_string())
        }
        DbErr::TryIntoErr { from, into, source } => RepositoryError::BadData(format!(
            "TryIntoErr: from={from:?} into={into:?} source={source:?}"
        )),
        DbErr::Conn(runtime_err) => RepositoryError::ConnectionFailed(runtime_err.to_string()),
        DbErr::Exec(runtime_err) => RepositoryError::Infrastructure(runtime_err.to_string()),
        DbErr::Query(runtime_err) => RepositoryError::Infrastructure(runtime_err.to_string()),
        DbErr::ConvertFromU64(_) => RepositoryError::BadData("ConvertFromU64".to_string()),
        DbErr::UnpackInsertId => RepositoryError::BadData("UnpackInsertId".to_string()),
        DbErr::UpdateGetPrimaryKey => RepositoryError::BadData("UpdateGetPrimaryKey".to_string()),
        DbErr::RecordNotFound(_) => RepositoryError::NotFound("RecordNotFound".to_string()),
        DbErr::AttrNotSet(_) => RepositoryError::BadData("AttrNotSet".to_string()),
        DbErr::Custom(_) => RepositoryError::BadData("Custom".to_string()),
        DbErr::Type(_) => RepositoryError::BadData("Type".to_string()),
        DbErr::Json(_) => RepositoryError::BadData("Json".to_string()),
        DbErr::Migration(_) => RepositoryError::BadData("Migration".to_string()),
        DbErr::RecordNotInserted => RepositoryError::BadData("RecordNotInserted".to_string()),
        DbErr::RecordNotUpdated => RepositoryError::BadData("RecordNotUpdated".to_string()),
    }
}

pub fn db_2_query_error(err: DbErr) -> QueryError {
    warn!(error=%err, "db error");
    QueryError::DbError(err.to_string())
}
