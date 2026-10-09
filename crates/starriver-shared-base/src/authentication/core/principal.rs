/// 主体：能够进行身份验证的用户或应用程序
pub trait Principal: Send + Sync {
    type Id;

    fn id(&self) -> &Self::Id;
}
