use sea_orm::{ConnectionTrait, TransactionTrait};
use starriver_blogging_domain::attachment::{
    factory::AttachmentFactory, file_type_checker::FileTypeChecker,
};
use starriver_shared_base::{
    io::{AsyncReader, AsyncWriter},
    upload_file::UploadLocationResolver,
};
use uuid::Uuid;

use crate::{
    dto::attachment_dto::res::AttachmentDto, error::CtxError,
    port::attachment_repository::AttachmentRepository,
};

pub struct AttachmentApplication<Conn, R, FC, UR> {
    conn: Conn,
    repo: R,
    factory: AttachmentFactory<FC>,
    upload_location_resolver: UR,
}

impl<Conn, R, FC, UR> AttachmentApplication<Conn, R, FC, UR>
where
    Conn: ConnectionTrait + TransactionTrait,
    R: AttachmentRepository,
    FC: FileTypeChecker,
    UR: UploadLocationResolver,
{
    pub fn new(
        conn: Conn,
        repo: R,
        factory: AttachmentFactory<FC>,
        upload_location_resolver: UR,
    ) -> Self {
        Self {
            conn,
            repo,
            factory,
            upload_location_resolver,
        }
    }

    pub async fn upload(
        &self,
        attachment_id: Uuid,
        claimed_extension: &str,
        mut async_reader: impl AsyncReader,
        mut async_writer: impl AsyncWriter,
    ) -> Result<AttachmentDto, CtxError> {
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
            return Err(CtxError::InvalidInput(
                "file too small for MIME detection".to_string(),
            ));
        }
        let attachment =
            self.factory
                .create_attachment(attachment_id, &magic_checker_buf, claimed_extension)?;
        let attachment = self.repo.insert(&self.conn, attachment).await?;
        let file_name = attachment.file_name();
        let url = self.upload_location_resolver.url(&file_name);
        let fields = attachment.dissolve();
        Ok(AttachmentDto {
            id: fields.0,
            file_name,
            url,
        })
    }
}
