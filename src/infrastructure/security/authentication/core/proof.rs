pub trait Proof {
    type Id;

    fn id(&self) -> &Self::Id;

    /// 请求的详情，有时候需要用其来辅助认证的过程
    fn request_details(&self) -> RequestDetails;
}

/// 认证请求的详情，通常是记录HTTP请求中的信息
#[derive(Debug)]
pub struct RequestDetails {}
