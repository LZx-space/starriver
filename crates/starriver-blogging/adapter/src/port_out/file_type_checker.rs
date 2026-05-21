use starriver_blogging_domain::{
    attachment::file_type_checker::FileTypeChecker, shared_error::DomainError,
};

pub struct DefaultFileTypeChecker {}

impl FileTypeChecker for DefaultFileTypeChecker {
    const MAGIC_CHECKER_HEADER_SIZE: usize = 100;

    fn check(&self, header: &[u8], claimed_mime_type: &str) -> Result<bool, DomainError> {
        if let Some(file_type) = infer::get(header) {
            Ok(file_type.mime_type() == claimed_mime_type)
        } else {
            Ok(false)
        }
    }
}
