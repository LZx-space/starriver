use crate::{
    middleware::authentication::{
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
        },
    },
    principal::{Auth, AuthenticatedUser},
};
use axum::{
    Form,
    body::Body,
    extract::FromRequest,
    http::{Method, Request, StatusCode, header},
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::Cookie;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use starriver_shared_base::authentication::UsernamePasswordCredentials;
use tracing::{error, info, warn};

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

impl Credentials for UsernamePasswordCredentials {}

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

////////////////////////////////////////////////////////////////////////////////

const AUTHENTION_TOKEN_COOKIE_NAME: &str = "token";

pub struct DefaultAuthenticationSuccessHandler {
    cfg: Auth,
}

impl DefaultAuthenticationSuccessHandler {
    pub fn new(cfg: Auth) -> Self {
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
        warn!(error = %err, "authentication failed");
        let (cause, message) = match err {
            AuthenticationError::UserLocked => (StatusCode::BAD_REQUEST, "user locked"),
            AuthenticationError::UserDisabled => (StatusCode::BAD_REQUEST, "user disabled"),
            AuthenticationError::InnerError => (StatusCode::INTERNAL_SERVER_ERROR, "inner error"),
            _ => (StatusCode::BAD_REQUEST, "username or password incorrect"),
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
            .map_err(|_| AuthenticationError::InnerError)?;
        info!(username = %form.0.username, "login credentials received");
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

    use crate::middleware::authentication::{
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
