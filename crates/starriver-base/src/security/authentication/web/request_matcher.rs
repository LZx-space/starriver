/// 请求匹配器
pub trait RequestMatcher {
    type Request;

    fn matches(&self, request: &Self::Request) -> impl Future<Output = bool> + Send;
}
