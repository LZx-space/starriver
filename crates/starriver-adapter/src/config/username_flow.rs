use crate::config::user_principal::{User, UserAuthenticator, UsernamePasswordCredential};

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
        core::credential::{AuthenticationContext, RequestMetadata},
        web::flow::AuthenticationFlow,
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
use tracing::{error, info};

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
        async move {
            path.starts_with("/static").not()
                && path.eq("/users").not()
                && method.eq(&Method::POST).not()
        }
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
        async move {
            // 提取表单数据
            let form = Form::<FormLoginCmd>::from_request(req, &())
                .await
                .map_err(|_| AuthenticationError::Unknown)?;
            info!(name: "login", "form login cmd: {:?}", form.0);
            // 创建凭证
            let credential = UsernamePasswordCredential::new(form.0.username, form.0.password)
                .map_err(|e| {
                    error!(name: "login", "new credential error: {}", e);
                    e
                })?;

            // 创建认证上下文
            let ctx = AuthenticationContext::new(credential, RequestMetadata::default());
            Ok(ctx)
        }
    }

    fn on_unauthenticated(&self, _req: Self::Request) -> impl Future<Output = Self::Response> {
        async {
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap_or_else(|e| {
                    error!("build unauthenticated response error: {}", e);
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap()
                })
        }
    }

    fn on_authenticate_success(
        &self,
        _req: &AuthenticationContext<Self::Credential>,
        principal: User,
    ) -> impl Future<Output = Self::Response> {
        async move {
            // 序列化用户信息
            let json = match serde_json::to_string(&principal) {
                Ok(json) => json,
                Err(e) => {
                    error!("serialize principal error: {}", e);
                    return Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap();
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
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap()
                })
        }
    }

    fn on_authenticate_failure(
        &self,
        _req: &AuthenticationContext<Self::Credential>,
        err: AuthenticationError,
    ) -> impl Future<Output = Self::Response> {
        async move {
            let error = ApiError::new(Cause::ClientBadRequest, err.to_string());
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
        let response = flow.on_unauthenticated(req).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
