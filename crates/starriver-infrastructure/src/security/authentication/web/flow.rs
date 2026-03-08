use crate::security::authentication::core::authenticator::{AuthenticationError, Authenticator};
use crate::security::authentication::core::credential::Credential;
use crate::security::authentication::core::principal::Principal;

pub trait AuthenticationFlow {
    type Request;

    type Response;

    type Credential: Credential;

    type Principal: Principal;

    type Authenticator: Authenticator<Credential = Self::Credential, Principal = Self::Principal>
        + Sync;

    fn is_authenticate_request(&self, req: &Self::Request) -> impl Future<Output = bool> + Send;

    fn is_access_require_authentication(
        &self,
        req: &Self::Request,
    ) -> impl Future<Output = bool> + Send;

    fn is_authenticated(&self, req: &Self::Request) -> impl Future<Output = bool> + Send;

    /// Extracts the credential from the request. if the request is authentication request
    fn extract_credential(
        &self,
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Credential, AuthenticationError>> + Send;

    fn authenticate(
        &self,
        authenticator: &Self::Authenticator,
        credential: &Self::Credential,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send {
        async move { authenticator.authenticate(&credential).await }
    }

    fn on_unauthenticated(&self, req: Self::Request)
    -> impl Future<Output = Self::Response> + Send;

    fn on_authenticate_success(
        &self,
        principal: Self::Principal,
    ) -> impl Future<Output = Self::Response> + Send;

    fn on_authenticate_failure(
        &self,
        err: AuthenticationError,
    ) -> impl Future<Output = Self::Response> + Send;
}
