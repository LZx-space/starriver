use crate::authentication::core::{
    authentication_request::AuthenticationRequest, error::AuthenticationError,
};

pub trait AuthenticationRequestExtractor {
    type Request;

    type AuthenticationRequest: AuthenticationRequest;

    fn extract(
        &self,
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::AuthenticationRequest, AuthenticationError>> + Send;
}
