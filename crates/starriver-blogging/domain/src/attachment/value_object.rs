use crate::shared_error::DomainError;

#[derive(Clone)]
pub struct FileSize(pub(crate) i64);
impl FileSize {
    /// 附件大小上限（字节）：10 MB
    pub const MAX_SIZE: i64 = 1024 * 1024 * 10;

    /// 大小规则的单点来源：已写入 `written` 字节后，是否还能容纳 `additional` 字节。
    /// 流式写入用它做实时判断，构造时用 `new`（等价于 `allows(0, size)`）做最终校验。
    pub fn allows(written: i64, additional: i64) -> bool {
        written + additional <= Self::MAX_SIZE
    }

    pub fn new(size: i64) -> Result<Self, DomainError> {
        if !Self::allows(0, size) {
            return Err(DomainError::AttachmentFileSizeInvalid(size));
        }
        Ok(Self(size))
    }

    pub fn size(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension(pub(crate) String);

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
