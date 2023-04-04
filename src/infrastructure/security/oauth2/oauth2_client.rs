// 按OAuth2.1协议实现OAuth2客户端

struct RegisteredClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scopes: Vec<String>,
    // todo 重定向端点
}

/// 授权服务器端点信息
struct AuthorizeEndpoint {
    // todo 授权码端点
    // todo 令牌端点
    // todo 用户API、用户端点（OIDC）
}

enum GrantTypes {
    Authorization_code,
    client_credentials,
    refresh_token,
}
// todo 认证失败处理器，多阶段trait公用
// todo 认证成功处理器，多阶段trait公用

struct AuthorizationRequest {
    authorization_endpoint_uri: String,
    response_type: String,
    client_id: String,
    code_challenge: Option<String>,
    redirect_uri: Option<String>,
    scope: Option<Vec<String>>,
    state: Option<String>,
}

struct TokenRequest {
    client_id: String,
    grant_type: String,
}
