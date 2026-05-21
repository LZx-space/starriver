use crate::middleware::authentication::core::{
    credentials::Credentials, error::AuthenticationError, principal::Principal,
};

pub trait Authenticator {
    type Credentials: Credentials;

    type Principal: Principal;

    /// 认证
    fn authenticate(
        &self,
        credentials: &Self::Credentials,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send;
}
