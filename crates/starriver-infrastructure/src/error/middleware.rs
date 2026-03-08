use axum::{
    BoxError,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use tracing::error;

use crate::error::error::{ApiError, Cause};

pub async fn handle_middleware_error(request_headers: HeaderMap, error: BoxError) -> Response {
    error!(name: "middleware error", "error：{}, headers: {:#?}", error, request_headers);
    ApiError::new(Cause::InnerError, error.to_string()).into_response()
}
