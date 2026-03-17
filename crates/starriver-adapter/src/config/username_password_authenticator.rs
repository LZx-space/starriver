use std::sync::Arc;

use starriver_application::user_service::UserApplication;
use starriver_infrastructure::security::authentication::_default_impl::{
    AuthenticatedUser, UsernamePasswordCredentials,
};
use starriver_infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};

pub struct UsernamePasswordAuthenticator {
    pub user_service: Arc<UserApplication>,
}

impl Authenticator for UsernamePasswordAuthenticator {
    type Credentials = UsernamePasswordCredentials;
    type Principal = AuthenticatedUser;

    async fn authenticate(
        &self,
        credentials: &Self::Credentials,
    ) -> Result<Self::Principal, AuthenticationError> {
        self.user_service.authenticate(credentials).await
    }
}
