use sea_orm::DbErr;
use starriver_domain::common_error::RepositoryError;

pub fn map_db_error(err: DbErr) -> RepositoryError {
    match err {
        DbErr::ConnectionAcquire(conn_acquire_err) => {
            RepositoryError::ConnectionFailed(conn_acquire_err.to_string())
        }
        DbErr::TryIntoErr { from, into, source } => RepositoryError::Infrastructure(format!(
            "from {} to {}, source: {}",
            from, into, source
        )),
        DbErr::Conn(runtime_err) => RepositoryError::ConnectionFailed(runtime_err.to_string()),
        DbErr::Exec(runtime_err) => RepositoryError::Infrastructure(runtime_err.to_string()),
        DbErr::Query(runtime_err) => RepositoryError::Infrastructure(runtime_err.to_string()),
        DbErr::ConvertFromU64(_) => {
            RepositoryError::Infrastructure("ConvertFromU64 error".to_string())
        }
        DbErr::UnpackInsertId => {
            RepositoryError::Infrastructure("UnpackInsertId error".to_string())
        }
        DbErr::UpdateGetPrimaryKey => {
            RepositoryError::Infrastructure("UpdateGetPrimaryKey error".to_string())
        }
        DbErr::RecordNotFound(err) => RepositoryError::NotFound(err),
        DbErr::AttrNotSet(_) => RepositoryError::Infrastructure("AttrNotSet error".to_string()),
        DbErr::Custom(_) => RepositoryError::Infrastructure("Custom error".to_string()),
        DbErr::Type(_) => RepositoryError::Infrastructure("Type error".to_string()),
        DbErr::Json(_) => RepositoryError::Infrastructure("Json error".to_string()),
        DbErr::Migration(_) => RepositoryError::Infrastructure("Migration error".to_string()),
        DbErr::RecordNotInserted => {
            RepositoryError::Infrastructure("RecordNotInserted error".to_string())
        }
        DbErr::RecordNotUpdated => {
            RepositoryError::Infrastructure("RecordNotUpdated error".to_string())
        }
    }
}
