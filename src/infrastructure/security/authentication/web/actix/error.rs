use actix_web::http::StatusCode;
use actix_web::ResponseError;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ErrUnauthorized {}

impl Display for ErrUnauthorized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ResponseError for ErrUnauthorized {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}
