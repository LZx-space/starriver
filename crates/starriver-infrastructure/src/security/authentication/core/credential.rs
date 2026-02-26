use axum::http::{HeaderMap, HeaderValue};

/// 用于认证的凭证<br>
/// 举例: 用户名&密码、OAuth2访问令牌
pub trait Credential: Send + Sync {}

/// 认证上下文
pub struct AuthenticationContext<Credential> {
    pub credential: Credential,
    pub request_metadata: RequestMetadata,
}

impl<Credential> AuthenticationContext<Credential> {
    pub fn new(credential: Credential, request_metadata: RequestMetadata) -> Self {
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
