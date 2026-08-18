use uuid::Uuid;

use crate::{
    attachment::{
        entity::Attachment,
        file_type_checker::FileTypeChecker,
        value_object::{Extension, FileSize},
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

    pub fn create_attachment(
        &self,
        attachment_id: Uuid,
        bytes: &[u8],
        extension: &Extension,
        file_size: FileSize,
    ) -> Result<Attachment, DomainError> {
        let checked = self.file_type_checker.check(bytes, extension.as_str())?;
        if !checked {
            return Err(DomainError::AttachmentExtensionInvalid(
                extension.as_str().to_string(),
            ));
        }

        Ok(Attachment::new(attachment_id, extension.clone(), file_size))
    }
}
