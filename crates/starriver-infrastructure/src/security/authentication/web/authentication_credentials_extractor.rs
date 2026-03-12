use crate::security::authentication::core::{
    authenticator::AuthenticationError, credentials::Credentials,
};

pub trait CredentialsExtractor {
    type Request;

    type Credentials: Credentials;

    fn extract(
        &self,
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Credentials, AuthenticationError>> + Send;
}
