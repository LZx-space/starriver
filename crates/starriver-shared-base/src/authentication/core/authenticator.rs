use crate::authentication::core::{
    authentication_request::AuthenticationRequest, authentication_result::AuthenticationResult,
    error::AuthenticationError, principal::Principal,
};

pub trait Authenticator {
    type Request: AuthenticationRequest;

    type Principal: Principal;

    fn authenticate(
        &self,
        req: &Self::Request,
    ) -> impl Future<Output = Result<AuthenticationResult<Self::Principal>, AuthenticationError>> + Send;
}
