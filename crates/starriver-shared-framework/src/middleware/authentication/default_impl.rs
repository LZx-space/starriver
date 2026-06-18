use axum::{
    Form,
    body::Body,
    extract::{FromRef, FromRequest, FromRequestParts},
    http::{Method, Request, StatusCode, header},
    response::{IntoResponse, Response},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use http::request::Parts;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use starriver_shared_base::{
    authentication::{PrincipalClaims, UsernamePasswordCredentials},
    middleware::authentication::{
        core::{
            error::AuthenticationError,
            principal::{Principal, SimpleAuthority},
        },
        web::{
            authentication_credentials_extractor::CredentialsExtractor,
            authentication_result_handler::{
                AuthenticationFailureHandler, AuthenticationSuccessHandler,
            },
            request_matcher::RequestMatcher,
        },
    },
};
use tracing::{error, info};

use core::time::Duration;
use starriver_shared_base::middleware::authentication::web::timing_attack_protection::TimingAttackProtection;
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

#[derive(Serialize)]
pub struct AuthenticatedUser(pub PrincipalClaims);

impl Principal for AuthenticatedUser {
    type Id = String;
    type Authority = SimpleAuthority;

    fn id(&self) -> &Self::Id {
        &self.0.username
    }

    fn authorities(&self) -> Vec<&Self::Authority> {
        vec![]
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
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

        decode::<PrincipalClaims>(
            jws,
            &DecodingKey::from_secret(cfg.jws_secret_as_ref()),
            &Validation::default(),
        )
        .map(|data| {
            let principal_claims = data.claims;
            AuthenticatedUser(principal_claims)
        })
        .map_err(|e| {
            error!(error = %e, "JWS token decode failed");
            StatusCode::UNAUTHORIZED
        })
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

    type Principal = AuthenticatedUser;

    async fn on_authentication_success(&self, principal: AuthenticatedUser) -> Self::Response {
        // 创建JWS声明
        let principal_claims = principal.0;

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
                return "".into_response();
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
                "build authentication success response error".into_response()
            })
    }
}

// ----------------------------------------------------------------------------------------

pub struct DefaultAuthenticationFailureHandler {}

impl AuthenticationFailureHandler for DefaultAuthenticationFailureHandler {
    type Response = Response;

    async fn on_authentication_failure(&self, err: AuthenticationError) -> Self::Response {
        info!(error=%err, "authentication failed");
        let (cause, message) = match err {
            AuthenticationError::UserLocked => (StatusCode::BAD_REQUEST, "user locked".to_string()),
            AuthenticationError::UserDisabled => {
                (StatusCode::BAD_REQUEST, "user disabled".to_string())
            }
            AuthenticationError::BadPassword => (
                StatusCode::BAD_REQUEST,
                "username or password incorrect".to_string(),
            ),
            AuthenticationError::InnerError { message } => {
                error!(error=%message, "authentication failed inner error");
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            _ => (
                StatusCode::BAD_REQUEST,
                "username or password incorrect".to_string(),
            ),
        };
        (cause, message).into_response()
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
            .map_err(|e| AuthenticationError::InnerError {
                message: e.to_string(),
            })?;
        info!(username = %form.0.username, "login credentials received");
        // 创建凭证
        let credentials = UsernamePasswordCredentials {
            username: form.0.username,
            password: form.0.password,
        };

        Ok(credentials)
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
    use starriver_shared_base::middleware::authentication::web::request_matcher::RequestMatcher;

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
