/// 用于认证的凭证
///
/// * 举例: 用户名&密码、OAuth2访问令牌
/// * 学术上 credential 一般指用户所声称 identity 的凭据。但其复数可以表达所有用于认证的信息
pub trait Credentials: Send + Sync {}
