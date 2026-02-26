use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use tracing::error;

use crate::error::error::{ApiError, PageError};

pub async fn handle_api_error(request_headers: HeaderMap, error: ApiError) -> Response {
    error!(name: "global api error handler", "error：{}, headers: {:#?}", error, request_headers);
    error.into_response()
}

pub async fn handle_page_error(request_headers: HeaderMap, error: PageError) -> Response {
    error!(name: "global page error handler", "error：{}, headers: {:#?}", error, request_headers);
    error.into_response()
}
