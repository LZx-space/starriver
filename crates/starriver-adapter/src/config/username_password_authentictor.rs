use std::sync::Arc;

use starriver_application::user_service::UserApplication;
use starriver_infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use starriver_infrastructure::security::authentication::username_password_authentication::{
    AuthenticatedUser, UsernamePasswordCredential,
};

pub struct UsernamePasswordAuthenticator {
    pub user_service: Arc<UserApplication>,
}

impl Authenticator for UsernamePasswordAuthenticator {
    type Credential = UsernamePasswordCredential;
    type Principal = AuthenticatedUser;

    fn authenticate(
        &self,
        credential: &Self::Credential,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send {
        async move { self.user_service.authenticate(credential).await }
    }
}
