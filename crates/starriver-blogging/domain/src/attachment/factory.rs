use uuid::Uuid;

use crate::{
    attachment::{
        entity::Attachment,
        file_type_checker::FileTypeChecker,
        value_object::{Extension, FileSize},
    },
    shared_error::DomainError,
};

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
        claimed_extension: &str,
    ) -> Result<Attachment, DomainError> {
        let checked = self.file_type_checker.check(bytes, claimed_extension)?;
        if !checked {
            return Err(DomainError::AttachmentExtensionInvalid(
                claimed_extension.to_string(),
            ));
        }

        let extension = Extension::new(claimed_extension)?;
        let file_size = FileSize::new(bytes.len() as i64)?;
        Ok(Attachment::new(attachment_id, extension, file_size))
    }
}
