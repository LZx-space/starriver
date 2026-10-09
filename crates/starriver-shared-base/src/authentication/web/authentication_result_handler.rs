use crate::authentication::core::{
    authentication_result::AuthenticationResult, error::AuthenticationError, principal::Principal,
};

pub trait AuthenticationSuccessHandler {
    type Response;

    type Principal: Principal;

    fn on_authentication_success(
        &self,
        result: AuthenticationResult<Self::Principal>,
    ) -> impl Future<Output = Self::Response> + Send;
}

pub trait AuthenticationFailureHandler {
    type Response;

    fn on_authentication_failure(
        &self,
        err: AuthenticationError,
    ) -> impl Future<Output = Self::Response> + Send;
}
