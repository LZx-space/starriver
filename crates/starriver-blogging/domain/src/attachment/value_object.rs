use crate::shared_error::DomainError;

#[derive(Clone)]
pub struct FileSize {
    size: i64,
}
impl FileSize {
    pub fn new(size: i64) -> Result<Self, DomainError> {
        if size > 1024 * 1024 * 10 {
            return Err(DomainError::AttachmentFileSizeInvalid(size));
        }
        Ok(Self { size })
    }

    pub fn size(&self) -> i64 {
        self.size
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension(String);

impl Extension {
    /// 允许的 MIME 类型白名单
    const ALLOWED_TYPES: &[&str] = &["png", "jpg", "jpeg", "gif"];

    pub fn new(extension: &str) -> Result<Self, DomainError> {
        // 检查是否在白名单中
        if !Self::ALLOWED_TYPES.contains(&extension) {
            return Err(DomainError::AttachmentExtensionInvalid(format!(
                "不允许的文件类型：{}",
                extension
            )));
        }
        Ok(Self(extension.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
