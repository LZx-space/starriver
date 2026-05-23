use axum::http::StatusCode;
use starriver_blogging_application::error::CtxError;
use starriver_shared_framework::response::ApiError;

pub fn map_error(error: CtxError) -> ApiError {
    match error {
        CtxError::InvalidInput(msg) => ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, msg),
        CtxError::NotFound(msg) => ApiError::new(StatusCode::NOT_FOUND, msg),
        CtxError::Conflict(msg) => ApiError::new(StatusCode::CONFLICT, msg),
        CtxError::Internal => ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
    }
}
