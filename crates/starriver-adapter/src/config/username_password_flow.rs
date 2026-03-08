use crate::config::username_password_authentictor::UsernamePasswordAuthenticator;

use axum::{
    body::Body,
    extract::FromRequest,
    http::{Method, Request, Response, StatusCode, header},
    response::IntoResponse,
};
use axum_extra::extract::{
    Form,
    cookie::{Cookie, CookieJar},
};
use serde::Deserialize;
use starriver_infrastructure::{
    error::error::ApiError,
    security::authentication::{
        username_password_authentication::{AuthenticatedUser, UsernamePasswordCredential},
        web::flow::AuthenticationFlow,
    },
};
use starriver_infrastructure::{
    error::error::Cause, security::authentication::core::authenticator::AuthenticationError,
};
use std::future::Future;
use std::ops::{Add, Not};
use time::{Duration, OffsetDateTime};
use tracing::{error, info, warn};

pub struct UsernamePasswordFlow {}

impl AuthenticationFlow for UsernamePasswordFlow {
    type Request = Request<Body>;
    type Response = Response<Body>;
    type Credential = UsernamePasswordCredential;
    type Principal = AuthenticatedUser;
    type Authenticator = UsernamePasswordAuthenticator;

    fn is_authenticate_request(&self, req: &Self::Request) -> impl Future<Output = bool> {
        let path = req.uri().path();
        let method = req.method();
        async { path.eq("/login") && method.eq(&Method::POST) }
    }

    fn is_access_require_authentication(&self, req: &Self::Request) -> impl Future<Output = bool> {
        let path = req.uri().path();
        let method = req.method();
        async { path.starts_with("/static").not() && method.eq(&Method::GET).not() }
    }

    fn is_authenticated(&self, req: &Self::Request) -> impl Future<Output = bool> {
        let cookies = CookieJar::from_headers(req.headers());
        async move { cookies.get("id").is_some() }
    }

    async fn extract_credential(
        &self,
        req: Self::Request,
    ) -> Result<Self::Credential, AuthenticationError> {
        // 提取表单数据
        let form = Form::<FormLoginCmd>::from_request(req, &())
            .await
            .map_err(|_| AuthenticationError::Unknown)?;
        info!(name: "login", "form login cmd: {:?}", form.0);
        // 创建凭证
        let credential = UsernamePasswordCredential {
            username: form.0.username,
            password: form.0.password,
        };

        // 创建认证上下文
        Ok(credential)
    }

    async fn on_unauthenticated(&self, _req: Self::Request) -> Self::Response {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap_or_else(|e| {
                error!("build unauthenticated response error: {}", e);
                ApiError::new(
                    Cause::InnerError,
                    "build unauthenticated response error".to_string(),
                )
                .into_response()
            })
    }

    fn on_authenticate_success(
        &self,
        principal: AuthenticatedUser,
    ) -> impl Future<Output = Self::Response> {
        async move {
            // 序列化用户信息
            let json = match serde_json::to_string(&principal) {
                Ok(json) => json,
                Err(e) => {
                    error!("serialize principal error: {}", e);
                    return ApiError::new(
                        Cause::InnerError,
                        "serialize principal error".to_string(),
                    )
                    .into_response();
                }
            };

            // 创建cookie
            let cookie = Cookie::build(("id", json))
                .http_only(true)
                .expires(OffsetDateTime::now_utc().add(Duration::hours(1)))
                .secure(false)
                .path("/")
                .build();

            // 构建响应
            Response::builder()
                .status(StatusCode::OK)
                .header(header::SET_COOKIE, cookie.to_string())
                .body(Body::empty())
                .unwrap_or_else(|e| {
                    error!("build authentication success response error: {}", e);
                    ApiError::new(
                        Cause::InnerError,
                        "build authentication success response error".to_string(),
                    )
                    .into_response()
                })
        }
    }

    fn on_authenticate_failure(
        &self,
        err: AuthenticationError,
    ) -> impl Future<Output = Self::Response> {
        async move {
            warn!("authentication failed: {}", err);
            let cause = match err {
                AuthenticationError::UsernameNotFound => Cause::ClientBadRequest,
                AuthenticationError::BadPassword => Cause::ClientBadRequest,
                _ => Cause::InnerError,
            };
            let message = match cause {
                Cause::InnerError => "authentication failed",
                _ => "username or password incorrect",
            };
            let error = ApiError::new(cause, message.to_string());
            error.into_response()
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct FormLoginCmd {
    pub username: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum_extra::extract::cookie::Cookie;
    use tokio;

    #[tokio::test]
    async fn test_is_access_require_authentication() {
        let flow = UsernamePasswordFlow {};

        // 测试需要认证的路径
        let req = Request::builder()
            .uri("/api/un_protected")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        assert!(!flow.is_access_require_authentication(&req).await);

        // 测试不需要认证的路径（/users POST）
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .body(Body::empty())
            .unwrap();
        assert!(flow.is_access_require_authentication(&req).await);
    }

    #[tokio::test]
    async fn test_is_authenticated() {
        let flow = UsernamePasswordFlow {};

        // 测试有cookie的情况
        let cookie = Cookie::new("id", "test_value");
        let req = Request::builder()
            .uri("/")
            .header("Cookie", cookie.to_string())
            .body(Body::empty())
            .unwrap();
        assert!(flow.is_authenticated(&req).await);

        // 测试没有cookie的情况
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        assert!(!flow.is_authenticated(&req).await);
    }

    #[tokio::test]
    async fn test_is_authenticate_request() {
        let flow = UsernamePasswordFlow {};

        // 测试登录请求
        let req = Request::builder()
            .uri("/login")
            .method(Method::POST)
            .body(Body::empty())
            .unwrap();
        assert!(flow.is_authenticate_request(&req).await);

        // 测试非登录请求
        let req = Request::builder()
            .uri("/login")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        assert!(!flow.is_authenticate_request(&req).await);
    }

    #[tokio::test]
    async fn test_on_unauthenticated() {
        let flow = UsernamePasswordFlow {};
        let req = Request::builder().body(Body::empty()).unwrap();
        let response = flow.on_unauthenticated(req).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
