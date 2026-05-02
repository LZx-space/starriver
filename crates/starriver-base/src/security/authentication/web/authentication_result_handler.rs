use crate::security::authentication::core::{
    authenticator::AuthenticationError, principal::Principal,
};

pub trait AuthenticationSuccessHandler {
    type Response;

    type Principal: Principal;

    fn on_authentication_success(
        &self,
        principal: Self::Principal,
    ) -> impl Future<Output = Self::Response> + Send;
}

pub trait AuthenticationFailureHandler {
    type Response;

    fn on_authentication_failure(
        &self,
        err: AuthenticationError,
    ) -> impl Future<Output = Self::Response> + Send;
}
