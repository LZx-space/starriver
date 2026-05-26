use std::fmt::Display;

use axum::{
    extract::{FromRef, FromRequest, FromRequestParts, Request},
    http::request::Parts,
    response::{IntoResponse, Response},
};

use http::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use validator::ValidationErrors;
use validator::{Validate, ValidateArgs};

use crate::response::ApiError;

/// 查询参数抽取器
pub struct Query<T>(pub T);

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 解析查询参数，错误自动转为 ApiError
        let query = axum::extract::Query::<T>::from_request_parts(parts, _state)
            .await
            .map_err(unified_valid_err)?;
        let value = query.0;
        value.validate().map_err(unified_valid_err2)?;
        Ok(Query(value))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

/// 路径参数抽取器
pub struct Path<T>(pub T);

impl<T, S> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 解析查询参数，错误自动转为 ApiError
        let path = axum::extract::Path::<T>::from_request_parts(parts, _state)
            .await
            .map_err(unified_bad_request_err)?;
        let value = path.0;
        Ok(Path(value))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

/// JSON参数抽取器
pub struct Json<T>(pub T);

impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(
        req: axum::extract::Request,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let json = axum::extract::Json::<T>::from_request(req, _state)
            .await
            .map_err(unified_valid_err)?;
        let value = json.0;
        value.validate().map_err(unified_valid_err2)?;
        Ok(Json(value))
    }
}

// ---------- 响应体实现 ----------
impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

/// JSON参数抽取器（支持上下文验证）
pub struct JsonEx<T>(pub T);

impl<State, T, CTX> FromRequest<State> for JsonEx<T>
where
    State: Send + Sync,
    CTX: FromRef<State>,
    T: for<'v> ValidateArgs<'v, Args = &'v CTX> + DeserializeOwned + Send,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &State) -> Result<Self, Self::Rejection> {
        // 1. 解析 JSON
        let json = axum::extract::Json::<T>::from_request(req, state)
            .await
            .map_err(unified_bad_request_err)?;

        // 2. 从 State 获取验证上下文（Patterns）
        let ctx = CTX::from_ref(state);

        // 3. 使用上下文进行验证
        let value = json.0;
        value.validate_with_args(&ctx).map_err(unified_valid_err2)?;

        Ok(JsonEx(value))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

///  multipart/form-data数据抽取器
pub struct Multipart(pub axum::extract::multipart::Multipart);

impl<S> FromRequest<S> for Multipart
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let multipart = axum::extract::multipart::Multipart::from_request(req, state)
            .await
            .map_err(unified_bad_request_err)?;
        Ok(Multipart(multipart))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////

fn unified_bad_request_err<E: Display>(e: E) -> ApiError {
    ApiError::new(StatusCode::BAD_REQUEST, e.to_string())
}

fn unified_valid_err<E: Display>(e: E) -> ApiError {
    ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, e.to_string())
}

fn unified_valid_err2(e: ValidationErrors) -> ApiError {
    let msg = serde_json::json!(e.errors());
    ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, msg.to_string())
}
