use crate::security::authentication::core::authenticator::{AuthenticationError, Authenticator};
use crate::security::authentication::core::credential::Credential;
use crate::security::authentication::core::principal::Principal;

pub trait AuthenticationFlow {
    type Request;

    type Response;

    type Credential: Credential;

    type Principal: Principal;

    type Authenticator: Authenticator<Credential = Self::Credential, Principal = Self::Principal>;

    fn is_access_require_authentication(&self, req: &Self::Request) -> bool;

    fn is_authenticated(&self, req: &Self::Request) -> bool;

    fn is_authenticate_request(&self, req: &Self::Request) -> bool;

    async fn extract_credential(
        &self,
        req: &mut Self::Request,
    ) -> Result<Self::Credential, AuthenticationError>;

    async fn authenticate(
        &self,
        authenticator: &Self::Authenticator,
        credential: &Self::Credential,
    ) -> Result<Self::Principal, AuthenticationError> {
        authenticator.authenticate(credential).await
    }

    async fn on_unauthenticated(
        &self,
        req: &Self::Request,
    ) -> Result<Self::Response, AuthenticationError>;

    async fn on_authenticate_success(
        &self,
        req: &Self::Request,
        principal: Self::Principal,
    ) -> Result<Self::Response, AuthenticationError>;

    async fn on_authenticate_failure(
        &self,
        req: &Self::Request,
        err: AuthenticationError,
    ) -> Result<Self::Response, AuthenticationError>;
}
