use serde::Deserialize;

use crate::authentication::core::authentication_request::AuthenticationRequest;

#[derive(Clone, Debug, Deserialize)]
pub struct IdentifierPasswordRequest {
    pub identifier: String,
    pub password: String,
}

impl AuthenticationRequest for IdentifierPasswordRequest {}
