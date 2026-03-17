use std::time::Instant;

use crate::{
    error::{ApiError, Cause},
    security::authentication::{
        core::{
            authenticator::AuthenticationError,
            credentials::Credentials,
            principal::{Principal, SimpleAuthority},
        },
        web::{
            authentication_credentials_extractor::CredentialsExtractor,
            authentication_result_handler::{
                AuthenticationFailureHandler, AuthenticationSuccessHandler,
            },
            request_matcher::RequestMatcher,
            timing_attack_protection::TimingAttackProtection,
        },
    },
};
use axum::{
    Form,
    body::Body,
    extract::{FromRequest, FromRequestParts},
    http::{Method, Request, StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use core::time::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::UtcDateTime;
use tokio::time::sleep;
use tracing::{error, info, warn};
use uuid::Uuid;

///////////////////////////////////////////////////////////////////////////////

pub struct LoginRequestMatcher {
    path: &'static str,
    method: Method,
}

impl RequestMatcher for LoginRequestMatcher {
    type Request = Request<Body>;

    fn matches(&self, request: &Self::Request) -> impl Future<Output = bool> + Send {
        let path = request.uri().path();
        let method = request.method();
        async move { self.path.eq(path) && self.method.eq(method) }
    }
}

impl Default for LoginRequestMatcher {
    fn default() -> Self {
        Self {
            path: "/login",
            method: Method::POST,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// 区别与用户提交的用户名&密码，该类型能包含更多的信心，比如IP等随HTTP请求携带的其他信息
#[derive(Clone, Debug)]
pub struct UsernamePasswordCredentials {
    pub username: String,
    pub password: String,
}

impl Credentials for UsernamePasswordCredentials {}

///////////////////////////////////////////////////////////////////////////////////////////////////

const AUTHENTION_TOKEN_COOKIE_NAME: &str = "token";

const AUTHENTICATION_JWS_SECRET: &str = "LZx";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub username: String,
    #[serde(default)]
    pub authorities: Vec<SimpleAuthority>,
}

impl Principal for AuthenticatedUser {
    type Id = String;
    type Authority = SimpleAuthority;

    fn id(&self) -> &Self::Id {
        &self.username
    }

    fn authorities(&self) -> Vec<&Self::Authority> {
        vec![]
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_infallible| StatusCode::UNAUTHORIZED)?;

        let jws = cookie_jar
            .get(AUTHENTION_TOKEN_COOKIE_NAME)
            .ok_or_else(|| {
                warn!("未找到认证凭证的Cookie");
                StatusCode::UNAUTHORIZED
            })?
            .value();

        decode::<PrincipalClaims>(
            jws,
            &DecodingKey::from_secret(AUTHENTICATION_JWS_SECRET.as_ref()),
            &Validation::default(),
        )
        .map(|data| {
            let principal_claims = data.claims;
            AuthenticatedUser {
                id: principal_claims.sub,
                username: principal_claims.username,
                authorities: principal_claims.authorities,
            }
        })
        .map_err(|e| {
            error!("解码JWS失败, {}", e);
            StatusCode::UNAUTHORIZED
        })
    }
}

///////////////////////////////////////////////////////////////////////////////

/// 异步运行时为tokio时，使用tokio的sleep函数实现延时以防止认证时的时差攻击
pub struct TokioTimingAttackProtection {
    pub delay: Duration,
}

impl TimingAttackProtection for TokioTimingAttackProtection {
    async fn fixed_duration_delay(&self, authenticate_start_at: Instant) {
        let elapsed = authenticate_start_at.elapsed();
        let to_delay = self.delay.saturating_sub(elapsed);
        if Duration::ZERO.eq(&to_delay) {
            return;
        }
        sleep(to_delay).await;
    }
}

impl Default for TokioTimingAttackProtection {
    fn default() -> Self {
        Self {
            delay: Duration::from_millis(500),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct PrincipalClaims {
    exp: i64,  // Expiration time (as UTC timestamp)
    nbf: i64,  // Not Before (as UTC timestamp)
    iat: i64,  // Issued at (as UTC timestamp)
    sub: Uuid, // Subject (whom token refers to)
    username: String,
    authorities: Vec<SimpleAuthority>,
}

pub struct DefaultAuthenticationSuccessHandler {}

impl AuthenticationSuccessHandler for DefaultAuthenticationSuccessHandler {
    type Response = Response;

    type Principal = AuthenticatedUser;

    async fn on_authentication_success(&self, principal: AuthenticatedUser) -> Self::Response {
        // 创建JWS声明
        let principal_claims = PrincipalClaims {
            exp: UtcDateTime::now()
                .saturating_add(time::Duration::hours(1))
                .unix_timestamp(),
            nbf: UtcDateTime::now().unix_timestamp(),
            iat: UtcDateTime::now().unix_timestamp(),
            sub: principal.id,
            username: principal.username,
            authorities: principal.authorities,
        };
        // 编码为JWS
        let jws = encode(
            &Header::default(),
            &principal_claims,
            &EncodingKey::from_secret(AUTHENTICATION_JWS_SECRET.as_ref()),
        );
        let jws = match jws {
            Ok(token) => token,
            Err(err) => {
                error!("serialize principal error: {}", err);
                return ApiError::new(Cause::InnerError, err.to_string()).into_response();
            }
        };
        // 创建cookie
        let cookie = Cookie::build((AUTHENTION_TOKEN_COOKIE_NAME, jws))
            .http_only(true)
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

// ----------------------------------------------------------------------------------------

pub struct DefaultAuthenticationFailureHandler {}

impl AuthenticationFailureHandler for DefaultAuthenticationFailureHandler {
    type Response = Response;

    async fn on_authentication_failure(&self, err: AuthenticationError) -> Self::Response {
        warn!("authentication failed: {}", err);
        let (cause, message) = match err {
            AuthenticationError::UserInactive => (Cause::Forbidden, "user inactive"),
            AuthenticationError::UserLocked => (Cause::Forbidden, "user locked"),
            AuthenticationError::UserDisabled => (Cause::Forbidden, "user disabled"),
            AuthenticationError::Unknown => (Cause::InnerError, "unknown error"),
            _ => (Cause::ClientBadRequest, "username or password incorrect"),
        };
        ApiError::new(cause, message).into_response()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize, Debug)]
pub struct FormLoginCmd {
    pub username: String,
    pub password: String,
}

pub struct DefaultCredentialsExtractor {}

impl CredentialsExtractor for DefaultCredentialsExtractor {
    type Request = Request<Body>;

    type Credentials = UsernamePasswordCredentials;

    async fn extract(&self, req: Self::Request) -> Result<Self::Credentials, AuthenticationError> {
        // 提取表单数据
        let form = Form::<FormLoginCmd>::from_request(req, &())
            .await
            .map_err(|_| AuthenticationError::Unknown)?;
        info!(name: "login", "form login cmd: {:?}", form.0);
        // 创建凭证
        let credentials = UsernamePasswordCredentials {
            username: form.0.username,
            password: form.0.password,
        };

        Ok(credentials)
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Method, Request},
    };

    use crate::security::authentication::{
        _default_impl::LoginRequestMatcher, web::request_matcher::RequestMatcher,
    };

    #[tokio::test]
    async fn test_is_authenticate_request() {
        let matcher = LoginRequestMatcher::default();

        // 测试登录请求
        let req = Request::builder()
            .uri("/login")
            .method(Method::POST)
            .body(Body::empty())
            .unwrap();
        assert!(matcher.matches(&req).await);

        // 测试非登录请求
        let req = Request::builder()
            .uri("/login")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        assert!(!matcher.matches(&req).await);
    }
}
