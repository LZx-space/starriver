use crate::{
    attachment::{
        entity::Attachment,
        file_type_checker::FileTypeChecker,
        value_object::{ClaimedExtension, Extension, FileSize},
    },
    shared_error::DomainError,
};

/// # Type Parameters
/// - `T`: file type checker.
pub struct AttachmentFactory<T> {
    file_type_checker: T,
}

impl<T: FileTypeChecker> AttachmentFactory<T> {
    pub fn new(file_type_checker: T) -> Self {
        Self { file_type_checker }
    }

    pub fn create(
        &self,
        header: &[u8],
        claimed_ext: ClaimedExtension,
        file_size: FileSize,
    ) -> Result<Attachment, DomainError> {
        let checked = self.file_type_checker.check(header, &claimed_ext)?;
        if !checked {
            return Err(DomainError::AttachmentExtensionInvalid(
                claimed_ext.as_str().to_string(),
            ));
        }
        let ext = Extension::from_claimed(claimed_ext);
        Ok(Attachment::new(ext, file_size))
    }
}
