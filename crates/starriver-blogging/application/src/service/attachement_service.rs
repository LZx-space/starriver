use starriver_blogging_domain::attachment::{
    entity::Attachment, factory::AttachmentFactory, file_type_checker::FileTypeChecker,
    repository::AttachmentRepository,
};
use starriver_shared_base::io::{AsyncReader, AsyncWriter};
use tracing::info;

use crate::error::CtxError;

pub struct AttachmentApplication<R, FC> {
    repo: R,
    factory: AttachmentFactory<FC>,
}

impl<R, FC> AttachmentApplication<R, FC>
where
    R: AttachmentRepository,
    FC: FileTypeChecker,
{
    pub fn new(repo: R, factory: AttachmentFactory<FC>) -> Self {
        Self { repo, factory }
    }

    pub async fn upload(
        &self,
        claimed_mime_type: &str,
        mut async_reader: impl AsyncReader,
        mut async_writer: impl AsyncWriter,
    ) -> Result<Attachment, CtxError> {
        let mut buf = [0u8; 4096];
        let mut magic_checker_buf = vec![0u8; FC::MAGIC_CHECKER_HEADER_SIZE];
        let mut magic_filled = 0; // 已收集的字节数
        loop {
            let n = async_reader.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            // 未收满时，从 buf 继续收集
            let remaining = FC::MAGIC_CHECKER_HEADER_SIZE - magic_filled;
            let to_copy = n.min(remaining);
            magic_checker_buf[magic_filled..magic_filled + to_copy]
                .copy_from_slice(&buf[..to_copy]);
            magic_filled += to_copy;
            async_writer.write(&buf[..n]).await?;
        }
        // 文件太小，不足以检测 MIME
        if magic_filled < FC::MAGIC_CHECKER_HEADER_SIZE {
            return Err(CtxError::Internal(
                "file too small for MIME detection".to_string(),
            ));
        }
        let attachment = self
            .factory
            .create_attachment(&magic_checker_buf, claimed_mime_type)?;
        info!(attachment_id=%attachment.id(), "attachment created");
        let attachment = self.repo.insert(attachment).await?;
        Ok(attachment)
    }
}
