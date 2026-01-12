use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::convert::Infallible;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

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

    pub fn new_with_client_reason(msg: String) -> Self {
        Self::new("A0000".to_string(), msg)
    }

    pub fn new_with_system_self_reason(msg: String) -> Self {
        Self::new("B0000".to_string(), msg)
    }

    pub fn new_with_third_part_reason(msg: String) -> Self {
        Self::new("C0000".to_string(), msg)
    }

    pub fn new_with_data<T: CodedErrData + 'static>(code: String, msg: String, data: T) -> Self {
        CodedErr::validate(&code);
        CodedErr {
            code,
            msg,
            data: Some(Box::new(data)),
        }
    }

    fn validate(code: &str) {
        if code.starts_with("A") || code.starts_with("B") || code.starts_with("C") {
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

pub trait CodedErrData: Display + Debug + Send + Sync {}

///
impl IntoResponse for CodedErr {
    fn into_response(self) -> Response {
        self.determine_http_status().into_response()
    }
}

impl Into<Infallible> for CodedErr {
    fn into(self) -> Infallible {
        panic!("Unexpected CodedErr: {}", self)
    }
}

impl From<Infallible> for CodedErr {
    fn from(_: Infallible) -> Self {
        // Infallible 表示永远不会出错，所以这个分支实际上永远不会被执行
        // 但如果真的执行了，说明有逻辑错误
        panic!("Unexpected Infallible error")
    }
}
