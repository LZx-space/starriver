use crate::security::authentication::core::authenticator::{AuthenticationError, Authenticator};
use crate::security::authentication::core::credential::{AuthenticationContext, Credential};
use crate::security::authentication::core::principal::Principal;

pub trait AuthenticationFlow {
    type Request;

    type Response;

    type Credential: Credential + Send + Sync;

    type Principal: Principal + Send + Sync;

    type Authenticator: Authenticator<Credential = Self::Credential, Principal = Self::Principal>
        + Sync;

    fn is_authenticate_request(&self, req: &Self::Request) -> impl Future<Output = bool> + Send;

    fn is_access_require_authentication(
        &self,
        req: &Self::Request,
    ) -> impl Future<Output = bool> + Send;

    fn is_authenticated(&self, req: &Self::Request) -> impl Future<Output = bool> + Send + Sync;

    /// Extracts the credential from the request. if the request is authentication request
    fn extract_credential(
        &self,
        req: Self::Request,
    ) -> impl Future<Output = Result<AuthenticationContext<Self::Credential>, AuthenticationError>>
    + Send
    + Sync;

    fn authenticate(
        &self,
        authenticator: &Self::Authenticator,
        ctx: &AuthenticationContext<Self::Credential>,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send {
        async move { authenticator.authenticate(&ctx).await }
    }

    fn on_unauthenticated(
        &self,
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, AuthenticationError>> + Send + Sync;

    fn on_authenticate_success(
        &self,
        ctx: &AuthenticationContext<Self::Credential>,
        principal: Self::Principal,
    ) -> impl Future<Output = Result<Self::Response, AuthenticationError>> + Send + Sync;

    fn on_authenticate_failure(
        &self,
        ctx: &AuthenticationContext<Self::Credential>,
        err: AuthenticationError,
    ) -> impl Future<Output = Result<Self::Response, AuthenticationError>> + Send + Sync;
}
