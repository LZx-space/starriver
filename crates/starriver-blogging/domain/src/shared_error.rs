use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("博文标题不能为空")]
    PostTitleIsEmpty,

    #[error("博文标题太长：{0}")]
    PostTitleTooLong(String),

    #[error("博文内容不能为空")]
    PostContentIsEmpty,

    #[error("博文内容太长：{0}")]
    PostContentTooLong(String),

    #[error("博文分类不能为空")]
    PostCategoryIsNone,

    #[error("博文分类名称太长：{0}")]
    PostCategoryTooLong(String),

    #[error("附件类型违规：{0}")]
    AttachmentMimeTypeInvalid(String),

    #[error("附件大小违规：{0}")]
    AttachmentFileSizeInvalid(u64),
}
