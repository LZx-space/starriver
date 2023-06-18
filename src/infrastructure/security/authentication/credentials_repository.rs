use crate::infrastructure::security::authentication::credentials::Credentials;

/// 认证凭证仓库
pub trait CredentialsRepository {
    type ID;
    fn find_by_id(&self, credentials_id: Self::ID) -> Option<Box<dyn Credentials>>;
}
