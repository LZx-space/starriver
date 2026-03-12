/// 用于认证的凭证
///
/// * 举例: 用户名&密码、OAuth2访问令牌
/// * 学术上 credential 在用户名密码登录场景下一般指password，但复数也会习惯来指两者
pub trait Credentials: Send + Sync {}
