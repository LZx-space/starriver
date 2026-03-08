use axum::http::{HeaderMap, HeaderValue};

/// 用于认证的凭证<br>
/// 举例: 用户名&密码、OAuth2访问令牌
pub trait Credential: Send + Sync {}

/// 认证上下文
pub struct AuthenticationContext<C: Credential> {
    pub credential: C,
    pub request_metadata: RequestMetadata,
}

impl<C: Credential> AuthenticationContext<C> {
    pub fn new(credential: C, request_metadata: RequestMetadata) -> Self {
        Self {
            credential,
            request_metadata,
        }
    }
}

/// 请求元数据，包含构建响应所需的所有信息
pub struct RequestMetadata {
    pub uri: String,
    pub method: String,
    pub client_ip: Option<String>,
    pub headers: HeaderMap<HeaderValue>,
}

impl Default for RequestMetadata {
    fn default() -> Self {
        Self {
            uri: String::new(),
            method: String::new(),
            client_ip: None,
            headers: HeaderMap::new(),
        }
    }
}
