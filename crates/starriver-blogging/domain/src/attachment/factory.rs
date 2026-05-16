use crate::{
    attachment::{
        entity::Attachment,
        file_type_checker::FileTypeChecker,
        value_object::{FileSize, MimeType},
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
        bytes: &[u8],
        claimed_extension: &str,
    ) -> Result<Attachment, DomainError> {
        self.file_type_checker.check(bytes, claimed_extension)?;
        let mime_type = MimeType::new_verified(claimed_extension)?;
        let file_size = FileSize::new(bytes.len() as u64)?;
        Ok(Attachment::new(mime_type, file_size))
    }
}
