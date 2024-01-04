use std::future::Future;

use crate::infrastructure::security::authentication::core::authenticator::AuthenticationError;
use crate::infrastructure::security::authentication::core::principal::Principal;
use crate::infrastructure::security::authentication::core::proof::Proof;

pub trait AuthenticationFlow {
    type Request;

    type Response;

    type Proof: Proof;

    type Principal: Principal;

    type ProofOutput: Future<Output=Result<Self::Proof, AuthenticationError>>;

    fn is_authenticated(&self, req: Self::Request) -> bool;

    fn on_unauthenticated(&self, req: Self::Request) -> Result<Self::Response, AuthenticationError>;

    fn is_authenticate_request(&self, req: Self::Request) -> bool;

    fn extract_proof(&self) -> Self::ProofOutput;

    fn authenticate(&self, proof: &Self::Proof) -> Result<Self::Principal, AuthenticationError>;

    fn on_success(&self) -> Result<Self::Response, AuthenticationError>;

    fn on_failure(&self) -> Result<Self::Response, AuthenticationError>;
}
