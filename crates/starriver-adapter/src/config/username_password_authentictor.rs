use std::sync::Arc;

use starriver_application::user_service::UserApplication;
use starriver_infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use starriver_infrastructure::security::authentication::core::credential::AuthenticationContext;
use starriver_infrastructure::security::authentication::username_password_authentication::{
    AuthenticatedUser, UsernamePasswordCredential,
};

pub struct UsernamePasswordAuthenticator {
    user_service: Arc<UserApplication>,
}

impl UsernamePasswordAuthenticator {
    pub fn new(user_service: Arc<UserApplication>) -> UsernamePasswordAuthenticator {
        UsernamePasswordAuthenticator { user_service }
    }
}

impl Authenticator for UsernamePasswordAuthenticator {
    type Credential = UsernamePasswordCredential;
    type Principal = AuthenticatedUser;

    fn authenticate(
        &self,
        ctx: &AuthenticationContext<Self::Credential>,
    ) -> impl Future<Output = Result<Self::Principal, AuthenticationError>> + Send {
        let credential = &ctx.credential;
        async move { self.user_service.authenticate(credential).await }
    }
}
