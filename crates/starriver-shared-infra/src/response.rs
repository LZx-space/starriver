use std::any::Any;

use axum::Json;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;
use tower_http::catch_panic::ResponseForPanic;

/// 统一错误响应体：`{"status": <code>, "message": "<文案>", "details": <可选结构化明细>}`
#[derive(Clone, Serialize)]
pub struct ApiError {
    status: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ApiError {
    pub fn new(status: StatusCode, message: String) -> Self {
        Self {
            status: status.into(),
            message,
            details: None,
        }
    }

    /// 带结构化明细的错误，例如字段级校验失败
    pub fn with_details(status: StatusCode, message: String, details: serde_json::Value) -> Self {
        Self {
            status: status.into(),
            message,
            details: Some(details),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_without_details() {
        let error = ApiError::new(StatusCode::BAD_REQUEST, "bad request".to_string());
        let json = serde_json::to_value(&error).expect("serializable");
        assert_eq!(
            json,
            serde_json::json!({"status": 400, "message": "bad request"})
        );
    }

    #[test]
    fn serializes_with_details() {
        let error = ApiError::with_details(
            StatusCode::UNPROCESSABLE_ENTITY,
            "validation failed".to_string(),
            serde_json::json!({"email": [{"code": "invalid_email"}]}),
        );
        let json = serde_json::to_value(&error).expect("serializable");
        assert_eq!(json["status"], 422);
        assert_eq!(json["details"]["email"][0]["code"], "invalid_email");
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////

/// panic 时的响应：panic 详情只进服务端日志，客户端收到与其它错误一致的 500。
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
            "internal server error".to_string(),
        )
        .into_response()
    }
}
