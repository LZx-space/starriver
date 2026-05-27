use std::convert::Infallible;

use axum::response::IntoResponse;
use axum::response::Response;
use http::StatusCode;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ApiError {
    status: u16,
    message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: String) -> Self {
        Self {
            status: status.into(),
            message,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, self.message).into_response()
    }
}

impl From<Infallible> for ApiError {
    fn from(error: Infallible) -> Self {
        Self::new(StatusCode::OK, error.to_string())
    }
}
