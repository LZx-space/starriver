use crate::infrastructure::security::authentication::core::credentials::Credentials;

/// 认证凭证仓库
pub trait CredentialsRepository<I, T: Credentials> {
    fn find_by_id(&self, credentials_id: &I) -> Option<T>;
}
