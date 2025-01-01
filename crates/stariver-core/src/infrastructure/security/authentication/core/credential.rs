/// 用户凭证<br>
/// 典型的: 用户名&密码、OAuth2访问令牌
pub trait Credential {
    /// 请求的详情，有时候需要用其来辅助认证的过程
    fn request_details(&self) -> RequestDetails;
}

/// 认证请求的详情，通常是记录HTTP请求中的信息
#[derive(Debug)]
pub struct RequestDetails {}
