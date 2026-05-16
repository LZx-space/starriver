use crate::shared_error::DomainError;

pub trait FileTypeChecker {
    fn check(&self, bytes: &[u8], claimed_extension: &str) -> Result<(), DomainError>;
}
