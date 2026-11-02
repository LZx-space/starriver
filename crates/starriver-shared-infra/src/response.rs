use std::any::Any;

use axum::Json;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;
use tower_http::catch_panic::ResponseForPanic;

/// 统一错误响应体：`{"message": "<文案>", "details": <可选结构化明细>}`
/// HTTP 状态码由响应状态行承载。
#[derive(Clone, Serialize)]
pub struct ApiError {
    #[serde(skip)]
    status: StatusCode,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ApiError {
    pub fn new(status: StatusCode, message: String) -> Self {
        Self {
            status,
            message: capitalize_first(message),
            details: None,
        }
    }

    /// 带结构化明细的错误，例如字段级校验失败
    pub fn with_details(status: StatusCode, message: String, details: serde_json::Value) -> Self {
        Self {
            status,
            message: capitalize_first(message),
            details: Some(details),
        }
    }
}

/// 英文文案按 sentence case 返回；中文/数字/符号开头原样保留。
fn capitalize_first(value: String) -> String {
    let Some(first) = value.chars().next() else {
        return value;
    };
    if !first.is_ascii_lowercase() {
        return value;
    }
    let mut result = String::with_capacity(value.len());
    result.push(first.to_ascii_uppercase());
    result.push_str(&value[first.len_utf8()..]);
    result
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

/// panic 时的响应：panic 详情只进服务端日志，客户端收到与其他错误一致的 500。
///
/// 用法：`CatchPanicLayer::custom(PanicResponse)`。
#[derive(Clone, Copy)]
pub struct PanicResponse;

impl ResponseForPanic for PanicResponse {
    type ResponseBody = Body;

    fn response_for_panic(
        &mut self,
        err: Box<dyn Any + Send + 'static>,
    ) -> Response<Self::ResponseBody> {
        let detail = if let Some(s) = err.downcast_ref::<String>() {
            s.as_str()
        } else if let Some(s) = err.downcast_ref::<&str>() {
            s
        } else {
            "unknown panic"
        };
        tracing::error!(panic = %detail, "request handling panicked");
        // 不向客户端泄漏 panic 细节
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
        .into_response()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_response_uses_http_status() {
        let response =
            ApiError::new(StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn serializes_without_details() {
        let error = ApiError::new(StatusCode::BAD_REQUEST, "Bad request".to_string());
        let json = serde_json::to_value(&error).expect("serializable");
        assert_eq!(json, serde_json::json!({"message": "Bad request"}));
    }

    #[test]
    fn serializes_with_details() {
        let error = ApiError::with_details(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Validation failed".to_string(),
            serde_json::json!({"email": [{"code": "invalid_email"}]}),
        );
        let json = serde_json::to_value(&error).expect("serializable");
        assert_eq!(json["message"], "Validation failed");
        assert_eq!(json["details"]["email"][0]["code"], "invalid_email");
        assert!(json.get("status").is_none());
    }

    #[test]
    fn capitalizes_first_ascii_letter() {
        assert_eq!(
            capitalize_first("failed to read stream".to_string()),
            "Failed to read stream"
        );
        assert_eq!(capitalize_first("Already".to_string()), "Already");
        assert_eq!(capitalize_first("中文".to_string()), "中文");
        assert_eq!(capitalize_first(String::new()), "");
    }
}
