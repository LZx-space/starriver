use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(Debug)]
pub struct CodedErr {
    code: String,
    msg: String,
    data: Option<Box<dyn CodedErrData>>,
}

impl CodedErr {
    pub fn new(code: String, msg: String) -> Self {
        CodedErr::validate(&code);
        CodedErr {
            code,
            msg,
            data: None,
        }
    }

    pub fn new_with_data<T: CodedErrData + 'static>(code: String, msg: String, data: T) -> Self {
        CodedErr::validate(&code);
        CodedErr {
            code,
            msg,
            data: Some(Box::new(data)),
        }
    }

    fn validate(code: &String) {
        let caused_by = code.as_str();
        if caused_by.starts_with("A") || caused_by.starts_with("B") || caused_by.starts_with("C") {
            return;
        }
        panic!("bad format error code")
    }

    pub fn determine_http_status(&self) -> StatusCode {
        if self.code.starts_with("A") {
            return StatusCode::BAD_REQUEST;
        } else if self.code.starts_with("B") {
            return StatusCode::INTERNAL_SERVER_ERROR;
        } else if self.code.starts_with("C") {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        StatusCode::OK
    }
}

impl Display for CodedErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.data {
            None => {
                write!(f, "{}, {}", self.code, self.msg)
            }
            Some(d) => {
                write!(f, "{}, {}, {}", self.code, self.msg, d)
            }
        }
    }
}

impl Error for CodedErr {}

pub trait CodedErrData: Display + Debug {}

///
impl ResponseError for CodedErr {
    fn status_code(&self) -> StatusCode {
        self.determine_http_status()
    }
}
