use serde::Deserialize;

use crate::authentication::core::authentication_request::AuthenticationRequest;

#[derive(Clone, Debug, Deserialize)]
pub struct IdentifierPasswordRequest {
    pub identifier: String,
    pub password: String,
}

impl AuthenticationRequest for IdentifierPasswordRequest {}

// -----------------------------------------------------------------------------

pub enum UserIdentifier<'a> {
    Username(&'a str),
    Email(&'a str),
}

impl<'a> UserIdentifier<'a> {
    pub fn new(identifier: &'a str) -> Self {
        if identifier.contains('@') {
            UserIdentifier::Email(identifier)
        } else {
            UserIdentifier::Username(identifier)
        }
    }
}
