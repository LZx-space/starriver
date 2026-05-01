use std::fmt::Display;

use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::{DbErr, TransactionError};
use serde::Serialize;
use strum::EnumIter;
use tracing::{error, warn};
use validator::ValidationError;

#[derive(Debug, Serialize)]
pub struct ApiError {
    cause: Cause,
    message: String,
}

impl ApiError {
    pub fn new<S: Display>(cause: Cause, message: S) -> Self {
        ApiError {
            cause,
            message: message.to_string(),
        }
    }

    pub fn with_data<S: Display>(cause: Cause, message: S) -> Self {
        ApiError {
            cause,
            message: message.to_string(),
        }
    }

    pub fn with_inner_error<S: Display>(message: S) -> Self {
        ApiError {
            cause: Cause::InnerError,
            message: message.to_string(),
        }
    }

    pub fn with_bad_request<S: Display>(message: S) -> Self {
        ApiError {
            cause: Cause::ClientBadRequest,
            message: message.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, code) = self.cause.to_http_status();
        if status_code.is_client_error() {
            warn!(code = %code, message = %self.message, "api error response");
        } else {
            error!(code = %code, message = %self.message, "api error response");
        }

        let json = ApiErrorResponse::<()> {
            code,
            message: self.message,
            data: None,
        }
        .into_response();
        (status_code, json).into_response()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<DbErr> for ApiError {
    fn from(err: DbErr) -> Self {
        ApiError::new(Cause::DbError, err.to_string())
    }
}

impl From<TransactionError<ApiError>> for ApiError {
    fn from(err: TransactionError<ApiError>) -> Self {
        ApiError::new(Cause::DbError, err.to_string())
    }
}

impl From<ValidationError> for ApiError {
    fn from(err: ValidationError) -> Self {
        ApiError::new(Cause::ValidationError, err.to_string())
    }
}

impl From<QueryRejection> for ApiError {
    fn from(err: QueryRejection) -> Self {
        ApiError::new(Cause::ClientBadRequest, err.to_string())
    }
}

impl From<JsonRejection> for ApiError {
    fn from(err: JsonRejection) -> Self {
        ApiError::new(Cause::ClientBadRequest, err.to_string())
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
        serde_json::to_string(&self)
            .unwrap_or_else(|_| {
                serde_json::json!({
                    "code": 500,
                    "message": "Failed to serialize response",
                    "data": null
                })
                .to_string()
            })
            .into_response()
    }
}

// ----------------------------------------------------------------------------------------
#[derive(Debug, Serialize, EnumIter)]
pub enum Cause {
    ClientBadRequest,
    ValidationError,
    Forbidden,
    DbError,
    InnerError,
    ThirdParty,
}

impl Cause {
    fn to_http_status(&self) -> (StatusCode, u16) {
        match self {
            Cause::ClientBadRequest => (StatusCode::BAD_REQUEST, 40001),
            Cause::ValidationError => (StatusCode::UNPROCESSABLE_ENTITY, 42201),
            Cause::Forbidden => (StatusCode::FORBIDDEN, 40301),
            Cause::DbError => (StatusCode::INTERNAL_SERVER_ERROR, 50001),
            Cause::InnerError => (StatusCode::INTERNAL_SERVER_ERROR, 50002),
            Cause::ThirdParty => (StatusCode::INTERNAL_SERVER_ERROR, 50003),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    pub fn ensure_correct_status_code() {
        Cause::iter().for_each(|cause| {
            let (status_code, code) = cause.to_http_status();
            assert!(
                status_code.is_client_error() || status_code.is_server_error(),
                "自定义的错误，仅该映射到400-499(客户端错误) || 500-599（服务器错误）"
            );
            assert!(
                code.to_string().starts_with(status_code.as_str()),
                "错误码必须以返回的Http状态码开头"
            );
        });
    }
}
