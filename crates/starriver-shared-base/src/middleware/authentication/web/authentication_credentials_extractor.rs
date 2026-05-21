use crate::middleware::authentication::core::{
    credentials::Credentials, error::AuthenticationError,
};

pub trait CredentialsExtractor {
    type Request;

    type Credentials: Credentials;

    fn extract(
        &self,
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Credentials, AuthenticationError>> + Send;
}
