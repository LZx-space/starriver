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
    #[must_use]
    pub fn new(identifier: &'a str) -> Self {
        // 统一去掉首尾空白：邮箱查询会 normalize，但用户名是精确匹配，
        let identifier = identifier.trim();
        if identifier.contains('@') {
            UserIdentifier::Email(identifier)
        } else {
            UserIdentifier::Username(identifier)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_surrounding_whitespace_before_classifying() {
        assert!(matches!(
            UserIdentifier::new("  alice  "),
            UserIdentifier::Username("alice")
        ));
        assert!(matches!(
            UserIdentifier::new(" alice@example.com "),
            UserIdentifier::Email("alice@example.com")
        ));
    }
}
