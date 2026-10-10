use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use starriver_blogging_application::error::CtxError;
use starriver_shared_framework::response::ApiError as Inner;

/// 本上下文唯一的 HTTP 错误出口：用例错误经 `From<CtxError>` 映射为状态码与文案。
/// handler 直接 `?` 抛出 `CtxError` 即可，无需逐点 `map_err`。
pub struct ApiError(Inner);

impl From<CtxError> for ApiError {
    fn from(error: CtxError) -> Self {
        let (status, message) = match error {
            CtxError::InvalidInput(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            CtxError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            CtxError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            CtxError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
        };
        Self(Inner::new(status, message))
    }
}

/// 传输层错误：multipart 解析失败
impl From<MultipartError> for ApiError {
    fn from(error: MultipartError) -> Self {
        Self(Inner::new(StatusCode::BAD_REQUEST, error.to_string()))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}
