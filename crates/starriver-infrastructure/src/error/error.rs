use std::{convert::Infallible, fmt::Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    cause: Cause,
    message: String,
}

// -------------------------------------------
impl ApiError {
    pub fn new(cause: Cause, message: String) -> Self {
        ApiError { cause, message }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.cause.to_http_status();
        (status_code, self.message).into_response()
    }
}

impl From<Infallible> for ApiError {
    fn from(_: Infallible) -> Self {
        ApiError::new(Cause::ClientBadRequest, "Infallible".to_string())
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.cause.to_http_status(), self.message)
    }
}

impl From<DbErr> for ApiError {
    fn from(err: DbErr) -> Self {
        ApiError::new(Cause::DbError, err.to_string())
    }
}

#[derive(Serialize)]
pub struct PageError {
    cause: Cause,
    message: String,
}

// -------------------------------------------
impl PageError {
    pub fn new(cause: Cause, message: String) -> Self {
        PageError { cause, message }
    }
}

impl IntoResponse for PageError {
    fn into_response(self) -> Response {
        let status_code = self.cause.to_http_status();
        if StatusCode::NOT_FOUND.eq(&status_code) {
            return Redirect::to("/static/404.html").into_response();
        } else {
            let msg = self.message;
            let uri = format!("{}{}", "/static/error.html?error=", msg);
            return Redirect::to(uri.as_str()).into_response();
        }
    }
}

impl From<Infallible> for PageError {
    fn from(_: Infallible) -> Self {
        PageError::new(Cause::ClientBadRequest, "Infallible".to_string())
    }
}

impl Display for PageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.cause.to_http_status(), self.message)
    }
}

impl From<DbErr> for PageError {
    fn from(err: DbErr) -> Self {
        PageError::new(Cause::DbError, err.to_string())
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

    use crate::error::error::ApiError;

    #[test]
    pub fn serialize_err() {
        let err_1 = ApiError::new(super::Cause::ClientBadRequest, "测试一下&str".to_string());
        println!("1-{:#?}", err_1.into_response());
        #[derive(serde::Serialize)]
        struct T {
            a: usize,
        }
        let t = T { a: 12 };
        let json = serde_json::to_string(&t).unwrap();
        let err_1 = ApiError::new(super::Cause::ClientBadRequest, json);
        println!("2-{:#?}", err_1.into_response());
    }
}
