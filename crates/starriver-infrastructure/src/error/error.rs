use std::{convert::Infallible, fmt::Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde::Serialize;
use strum::EnumIter;

#[derive(Serialize)]
pub struct ApiError {
    cause: Cause,
    message: String,
}

impl ApiError {
    pub fn new(cause: Cause, message: String) -> Self {
        ApiError { cause, message }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, code) = self.cause.to_http_status();
        let json = ApiErrorResponse::<()> {
            code: code,
            message: self.message,
            data: None,
        }
        .into_response();
        (status_code, json).into_response()
    }
}

impl From<Infallible> for ApiError {
    fn from(_: Infallible) -> Self {
        ApiError::new(Cause::ClientBadRequest, "Infallible".to_string())
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (status, code) = self.cause.to_http_status();
        write!(f, "({}, {}, {})", status, code, self.message)
    }
}

impl From<DbErr> for ApiError {
    fn from(err: DbErr) -> Self {
        ApiError::new(Cause::DbError, err.to_string())
    }
}

// ----------------------------------------------------------------------------------------
#[derive(Serialize)]
pub struct ApiErrorResponse<T: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> IntoResponse for ApiErrorResponse<T> {
    fn into_response(self) -> Response {
        let json_response = serde_json::to_string(&self).unwrap_or_else(|_| {
            serde_json::json!({
                "code": 500,
                "message": "Failed to serialize response",
                "data": null
            })
            .to_string()
        });

        (
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            json_response,
        )
            .into_response()
    }
}

// ----------------------------------------------------------------------------------------
#[derive(Serialize)]
pub struct PageError {
    cause: Cause,
    message: String,
}

impl IntoResponse for PageError {
    fn into_response(self) -> Response {
        todo!()
    }
}

impl Display for PageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (status, code) = self.cause.to_http_status();
        write!(f, "({}, {}, {})", status, code, self.message)
    }
}

// ----------------------------------------------------------------------------------------
#[derive(Serialize, EnumIter)]
pub enum Cause {
    ClientBadRequest,
    DbError,
    InnerError,
    ThirdParty,
}

impl Cause {
    fn to_http_status(&self) -> (StatusCode, u16) {
        match self {
            Cause::ClientBadRequest => (StatusCode::BAD_REQUEST, 40001),
            Cause::DbError => (StatusCode::INTERNAL_SERVER_ERROR, 50001),
            Cause::InnerError => (StatusCode::INTERNAL_SERVER_ERROR, 50002),
            Cause::ThirdParty => (StatusCode::INTERNAL_SERVER_ERROR, 50003),
        }
    }
}

#[cfg(test)]
mod test {
    use strum::IntoEnumIterator;

    use crate::error::error::Cause;

    #[test]
    pub fn ensure_correct_status_code() {
        Cause::iter().for_each(|cause| {
            let (status_code, code) = cause.to_http_status();
            assert!(
                status_code.is_client_error() || status_code.is_server_error(),
                "自定义的错误，仅该映射到400-499(客户端错误) || 500-599（服务器错误）"
            );
            assert!(code.to_string().starts_with(status_code.as_str()), "c");
        });
    }
}
