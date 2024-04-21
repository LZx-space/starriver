use std::future::Future;

use crate::infrastructure::security::authentication::core::authenticator::AuthenticationError;
use crate::infrastructure::security::authentication::core::credential::Credential;
use crate::infrastructure::security::authentication::core::principal::Principal;

pub trait AuthenticationFlow {
    type Request;

    type Response;

    type Credential: Credential;

    type Principal: Principal;

    type CredentialOutput: Future<Output = Result<Self::Credential, AuthenticationError>>;

    fn is_authenticated(&self, req: &Self::Request) -> bool;

    async fn on_unauthenticated(
        &self,
        req: &Self::Request,
    ) -> Result<Self::Response, AuthenticationError>;

    fn is_authenticate_request(&self, req: &Self::Request) -> bool;

    async fn extract_credential(&self, req: Self::Request) -> Self::CredentialOutput;

    async fn authenticate(
        &self,
        credential: &Self::Credential,
    ) -> Result<Self::Principal, AuthenticationError>;

    async fn on_success(
        &self,
        req: &Self::Request,
        principal: Self::Principal,
    ) -> Result<Self::Response, AuthenticationError>;

    async fn on_failure(
        &self,
        req: &Self::Request,
        e: AuthenticationError,
    ) -> Result<Self::Response, AuthenticationError>;
}
