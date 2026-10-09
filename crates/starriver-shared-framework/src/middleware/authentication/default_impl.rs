use axum::{
    Form,
    body::Body,
    extract::{FromRef, FromRequest, FromRequestParts},
    http::{Method, Request, StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use starriver_shared_base::authentication::{
    authentication_request::IdentifierPasswordRequest,
    core::{authentication_result::AuthenticationResult, error::AuthenticationError},
    principal::DefaultUser,
    web::{
        authentication_request_extractor::AuthenticationRequestExtractor,
        authentication_result_handler::{
            AuthenticationFailureHandler, AuthenticationSuccessHandler,
        },
        request_matcher::RequestMatcher,
    },
};
use time::UtcDateTime;
use tracing::{error, info};
use uuid::Uuid;

use core::time::Duration;
use starriver_shared_base::authentication::web::timing_attack_protection::TimingAttackProtection;
use std::{sync::Arc, time::Instant};
use tokio::time::sleep;

use crate::config::Auth;

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

#[derive(Deserialize, Serialize)]
pub struct AuthenticatedJwtClaims {
    exp: i64,      // Expiration time (as UTC timestamp)
    nbf: i64,      // Not Before (as UTC timestamp)
    iat: i64,      // Issued at (as UTC timestamp)
    pub sub: Uuid, // Subject (whom token refers to)
    pub username: String,
    pub email: String,
}

impl AuthenticatedJwtClaims {
    pub fn new(exp: time::Duration, id: Uuid, username: String, email: String) -> Self {
        let now = UtcDateTime::now();
        Self {
            exp: now.saturating_add(exp).unix_timestamp(),
            nbf: now.unix_timestamp(),
            iat: now.unix_timestamp(),
            sub: id,
            username,
            email,
        }
    }
}

impl<S> FromRequestParts<S> for AuthenticatedJwtClaims
where
    Arc<Auth>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cfg = Arc::<Auth>::from_ref(state);

        let cookie_jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_infallible| StatusCode::UNAUTHORIZED)?;

        let jws = cookie_jar
            .get(&cfg.jws_cookie_name)
            .ok_or_else(|| {
                info!("authentication cookie not found in request");
                StatusCode::UNAUTHORIZED
            })?
            .value();

        decode::<AuthenticatedJwtClaims>(
            jws,
            &DecodingKey::from_secret(cfg.jws_secret_as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| {
            error!(error = %e, "JWS token decode failed");
            StatusCode::UNAUTHORIZED
        })
    }
}

// impl ForeignTrait<T1..Tn> for T0，只要至少有一个 T0..Tn 是本地类型，就不违背孤儿规则
impl From<AuthenticatedJwtClaims> for DefaultUser {
    fn from(value: AuthenticatedJwtClaims) -> Self {
        DefaultUser::new(value.sub, value.username, value.email)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DefaultAuthenticationSuccessHandler {
    cfg: Arc<Auth>,
}

impl DefaultAuthenticationSuccessHandler {
    pub fn new(cfg: Arc<Auth>) -> Self {
        Self { cfg }
    }
}

impl AuthenticationSuccessHandler for DefaultAuthenticationSuccessHandler {
    type Response = Response;

    type Principal = DefaultUser;

    async fn on_authentication_success(
        &self,
        result: AuthenticationResult<DefaultUser>,
    ) -> Self::Response {
        // 创建JWS声明
        let user = result.principal();
        let principal_claims = AuthenticatedJwtClaims::new(
            time::Duration::hours(self.cfg.jws_exp_hours as i64),
            user.id,
            user.username.clone(),
            user.email.clone(),
        );

        // 编码为JWS
        let jws = encode(
            &Header::default(),
            &principal_claims,
            &EncodingKey::from_secret(self.cfg.jws_secret_as_ref()),
        );
        let jws = match jws {
            Ok(token) => token,
            Err(err) => {
                error!(error = %err, "failed to serialize JWS principal claims");
                return (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
                    .into_response();
            }
        };
        // 创建cookie
        let cookie = Cookie::build((&self.cfg.jws_cookie_name, jws))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .path("/")
            .build();

        // 构建响应
        Response::builder()
            .status(StatusCode::OK)
            .header(header::SET_COOKIE, cookie.to_string())
            .body(Body::empty())
            .unwrap_or_else(|e| {
                error!(error = %e, "failed to build authentication success response");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
            })
    }
}

// ----------------------------------------------------------------------------------------

pub struct DefaultAuthenticationFailureHandler {}

impl AuthenticationFailureHandler for DefaultAuthenticationFailureHandler {
    type Response = Response;

    async fn on_authentication_failure(&self, err: AuthenticationError) -> Self::Response {
        info!(error=%err, "authentication failed");
        match err {
            // 内部错误：状态码 500，但绝不向客户端泄漏内部细节
            AuthenticationError::InnerError { message } => {
                error!(error=%message, "authentication failed inner error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
            // 其余所有失败（未找到/空/密码错/锁定/禁用/删除）统一文案，防止账号状态枚举
            _ => (
                StatusCode::BAD_REQUEST,
                "username or password incorrect".to_string(),
            ),
        }
        .into_response()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////

pub struct DefaultAuthenticationRequestExtractor {}

impl AuthenticationRequestExtractor for DefaultAuthenticationRequestExtractor {
    type Request = Request<Body>;

    type AuthenticationRequest = IdentifierPasswordRequest;

    async fn extract(
        &self,
        req: Self::Request,
    ) -> Result<Self::AuthenticationRequest, AuthenticationError> {
        // 提取表单数据
        let form = Form::<IdentifierPasswordRequest>::from_request(req, &())
            .await
            .map_err(|e| AuthenticationError::InnerError {
                message: e.to_string(),
            })?;
        info!(identifier = %form.0.identifier, "login request received and parsed");
        Ok(form.0)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use starriver_shared_base::authentication::web::request_matcher::RequestMatcher;

    use crate::middleware::authentication::default_impl::LoginRequestMatcher;

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
