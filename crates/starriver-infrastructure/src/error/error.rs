use std::{convert::Infallible, fmt::Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Serialize)]
pub struct AppError {
    cause: Cause,
    message: String,
}

// -------------------------------------------
impl AppError {
    pub fn new(cause: Cause, message: String) -> Self {
        AppError { cause, message }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.cause.to_http_status();
        let body = serde_json::to_string(&self).unwrap();
        println!("body-{}", body);
        (status_code, body).into_response()
    }
}

impl From<Infallible> for AppError {
    fn from(_: Infallible) -> Self {
        AppError::new(Cause::ClientBadRequest, "Infallible".to_string())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.cause.to_http_status(), self.message)
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::new(Cause::DbError, err.to_string())
    }
}

// ------------------------------------------
#[derive(Serialize)]
pub enum Cause {
    ClientBadRequest,
    DbError,
    InnerError,
    ThirdParty,
}

impl Cause {
    fn to_http_status(&self) -> StatusCode {
        match self {
            Cause::ClientBadRequest => StatusCode::BAD_REQUEST,
            Cause::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            Cause::InnerError => StatusCode::INTERNAL_SERVER_ERROR,
            Cause::ThirdParty => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(test)]
mod test {
    use axum::response::IntoResponse;

    use crate::error::error::AppError;

    #[test]
    pub fn serialize_err() {
        let err_1 = AppError::new(super::Cause::ClientBadRequest, "测试一下&str".to_string());
        println!("1-{:#?}", err_1.into_response());
        #[derive(serde::Serialize)]
        struct T {
            a: usize,
        }
        let t = T { a: 12 };
        let json = serde_json::to_string(&t).unwrap();
        let err_1 = AppError::new(super::Cause::ClientBadRequest, json);
        println!("2-{:#?}", err_1.into_response());
    }
}
