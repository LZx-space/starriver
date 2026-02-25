use crate::config::user_principal::{User, UserAuthenticator, UsernamePasswordCredential};

use axum::{
    body::Body,
    http::{Method, Request, Response, StatusCode, header},
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;
use starriver_infrastructure::{
    error::error::AppError,
    security::authentication::{
        core::credential::AuthenticationContext, web::flow::AuthenticationFlow,
    },
};
use starriver_infrastructure::{
    error::error::Cause, security::authentication::core::authenticator::AuthenticationError,
};
use std::future::Future;
use std::{
    future::ready,
    ops::{Add, Not},
};
use time::{Duration, OffsetDateTime};
use tracing::error;

#[derive(Clone)]
pub struct UsernameFlow {}

impl AuthenticationFlow for UsernameFlow {
    type Request = Request<Body>;
    type Response = Response<Body>;
    type Credential = UsernamePasswordCredential;
    type Principal = User;
    type Authenticator = UserAuthenticator;

    fn is_authenticate_request(&self, req: &Self::Request) -> impl Future<Output = bool> + Send {
        ready(req.uri().path().eq("/login") && req.method().eq(&Method::POST))
    }

    fn is_access_require_authentication(&self, req: &Self::Request) -> impl Future<Output = bool> {
        let path = req.uri().path().to_owned();
        let method = req.method().to_owned();
        async move { path.eq("/users").not() && method.eq(&Method::POST).not() }
    }

    fn is_authenticated(&self, req: &Self::Request) -> impl Future<Output = bool> {
        let cookies = CookieJar::from_headers(req.headers());
        async move { cookies.get("id").is_some() }
    }

    fn extract_credential(
        &self,
        req: Self::Request,
    ) -> impl Future<Output = Result<AuthenticationContext<Self::Credential>, AuthenticationError>>
    {
        async move { todo!() }
    }

    fn on_unauthenticated(
        &self,
        _req: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, AuthenticationError>> {
        async {
            let response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .map_err(|e| {
                    error!("build unauthenticated response error: {}", e);
                    AuthenticationError::Unknown
                })?;
            Ok(response)
        }
    }

    fn on_authenticate_success(
        &self,
        _req: &AuthenticationContext<Self::Credential>,
        principal: User,
    ) -> impl Future<Output = Result<Self::Response, AuthenticationError>> {
        async move {
            // 序列化用户信息
            let json = serde_json::to_string(&principal).map_err(|e| {
                error!("serialize principal error: {}", e);
                AuthenticationError::Unknown
            })?;

            // 创建cookie
            let cookie = Cookie::build(("id", json))
                .http_only(true)
                .expires(OffsetDateTime::now_utc().add(Duration::hours(1)))
                .secure(false)
                .path("/")
                .build();

            // 构建响应
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::SET_COOKIE, cookie.to_string())
                .body(Body::empty())
                .map_err(|e| {
                    error!("build authentication success response error: {}", e);
                    AuthenticationError::Unknown
                })?;

            Ok(response)
        }
    }

    fn on_authenticate_failure(
        &self,
        _req: &AuthenticationContext<Self::Credential>,
        err: AuthenticationError,
    ) -> impl Future<Output = Result<Self::Response, AuthenticationError>> {
        async move {
            let coded_err = AppError::new(Cause::ClientBadRequest, err.to_string());
            let response = coded_err.into_response();
            Ok(response)
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
        let flow = UsernameFlow {};

        // 测试需要认证的路径
        let req = Request::builder()
            .uri("/api/protected")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        assert!(flow.is_access_require_authentication(&req).await);

        // 测试不需要认证的路径（/users POST）
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .body(Body::empty())
            .unwrap();
        assert!(!flow.is_access_require_authentication(&req).await);
    }

    #[tokio::test]
    async fn test_is_authenticated() {
        let flow = UsernameFlow {};

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
        let flow = UsernameFlow {};

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

        // 测试非登录路径
        let req = Request::builder()
            .uri("/other")
            .method(Method::POST)
            .body(Body::empty())
            .unwrap();
        assert!(!flow.is_authenticate_request(&req).await);
    }

    #[tokio::test]
    async fn test_on_unauthenticated() {
        let flow = UsernameFlow {};
        let req = Request::builder().body(Body::empty()).unwrap();
        let result = flow.on_unauthenticated(req).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
