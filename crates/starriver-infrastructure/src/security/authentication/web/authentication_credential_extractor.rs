use crate::security::authentication::core::{
    authenticator::AuthenticationError, credential::Credential,
};

pub trait CredentialExtractor {
    type Request;

    type Credential: Credential;

    fn extract(
        &self,
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Credential, AuthenticationError>> + Send;
}
