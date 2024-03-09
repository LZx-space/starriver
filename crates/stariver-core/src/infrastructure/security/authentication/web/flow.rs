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

    fn on_unauthenticated(
        &self,
        req: &Self::Request,
    ) -> Result<Self::Response, AuthenticationError>;

    fn is_authenticate_request(&self, req: &Self::Request) -> bool;

    fn extract_credential(&self, req: Self::Request) -> Self::CredentialOutput;

    fn authenticate(
        &self,
        credential: &Self::Credential,
    ) -> Result<Self::Principal, AuthenticationError>;

    fn on_success(
        &self,
        req: &Self::Request,
        principal: Self::Principal,
    ) -> Result<Self::Response, AuthenticationError>;

    fn on_failure(
        &self,
        req: &Self::Request,
        e: AuthenticationError,
    ) -> Result<Self::Response, AuthenticationError>;
}
