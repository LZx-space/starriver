use crate::shared_error::DomainError;

pub trait FileTypeChecker {
    const MAGIC_HEADER_MAX_BYTES: usize = 32;

    /// # params
    /// - `header`: the first few bytes of the file
    /// - `claimed_extension`: the extension claimed by the user
    ///
    /// # returns
    /// - `Ok(true)` if the file type matches the claimed extension
    /// - `Ok(false)` if the file type does not match the claimed extension
    /// - `Err` if an error occurs during the check
    fn check(&self, header: &[u8], claimed_extension: &str) -> Result<bool, DomainError>;
}
