use std::fmt::{Debug, Display, Formatter};

use actix_web::ResponseError;
use actix_web::http::StatusCode;

#[derive(Debug)]
pub struct ErrUnauthorized {}

impl Display for ErrUnauthorized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Unauthorized")
    }
}

impl ResponseError for ErrUnauthorized {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}
