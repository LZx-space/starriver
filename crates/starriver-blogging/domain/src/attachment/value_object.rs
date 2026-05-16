use crate::shared_error::DomainError;

pub struct FileSize {
    pub size: u64,
}
impl FileSize {
    pub fn new(size: u64) -> Result<Self, DomainError> {
        if size > 1024 * 1024 * 10 {
            return Err(DomainError::AttachmentFileSizeInvalid(size));
        }
        Ok(Self { size })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MimeType(String);

impl MimeType {
    /// 允许的 MIME 类型白名单
    const ALLOWED_TYPES: &[&str] = &["image/png", "image/jpeg", "image/gif"];

    pub fn new_verified(verified_extension: &str) -> Result<Self, DomainError> {
        // 检查是否在白名单中
        if !Self::ALLOWED_TYPES.contains(&verified_extension) {
            return Err(DomainError::AttachmentMimeTypeInvalid(format!(
                "不允许的文件类型：{}",
                verified_extension
            )));
        }
        Ok(Self(verified_extension.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
