use crate::security::authentication::core::authenticator::{AuthenticationError, Authenticator};
use crate::security::authentication::core::credential::Credential;
use crate::security::authentication::core::principal::Principal;

pub trait AuthenticationFlow {
    type Request;

    type Response;

    type Credential: Credential + Send + Sync;

    type Principal: Principal + Send + Sync;

    type Authenticator: Authenticator<Credential = Self::Credential, Principal = Self::Principal>
        + Sync;

    fn is_authenticate_request(&self, req: &Self::Request) -> bool;

    fn is_access_require_authentication(
        &self,
        req: &Self::Request,
    ) -> impl Future<Output = bool> + Send;

    fn is_authenticated(&self, req: &Self::Request) -> impl Future<Output = bool> + Send + Sync;

    fn extract_credential(
        &self,
        req: &mut Self::Request,
    ) -> impl Future<Output = Result<Self::Credential, AuthenticationError>> + Send + Sync;

    fn authenticate(
        &self,
        authenticator: &Self::Authenticator,
        credential: &Self::Credential,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send {
        async move { authenticator.authenticate(&credential).await }
    }

    fn on_unauthenticated(
        &self,
        req: &Self::Request,
    ) -> impl Future<Output = Result<Self::Response, AuthenticationError>> + Send + Sync;

    fn on_authenticate_success(
        &self,
        req: &Self::Request,
        principal: Self::Principal,
    ) -> impl Future<Output = Result<Self::Response, AuthenticationError>> + Send + Sync;

    fn on_authenticate_failure(
        &self,
        req: &Self::Request,
        err: AuthenticationError,
    ) -> impl Future<Output = Result<Self::Response, AuthenticationError>> + Send + Sync;
}
