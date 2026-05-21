use crate::shared_error::DomainError;

pub trait FileTypeChecker {
    const MAGIC_CHECKER_HEADER_SIZE: usize;

    /// # params
    /// - `header`: the first few bytes of the file
    /// - `claimed_mime_type`: the mime type claimed by the user
    fn check(&self, header: &[u8], claimed_mime_type: &str) -> Result<bool, DomainError>;
}
