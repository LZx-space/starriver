/// 认证凭证仓库
pub trait CredentialsRepository {
    type ID;
    type CredentialsType;
    fn find_by_id(&self, credentials_id: &Self::ID) -> Option<Box<Self::CredentialsType>>;
}
