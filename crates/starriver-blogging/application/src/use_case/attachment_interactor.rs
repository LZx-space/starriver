use starriver_blogging_domain::attachment::{
    factory::AttachmentFactory,
    file_type_checker::FileTypeChecker,
    value_object::{Extension, FileSize},
};
use starriver_shared_base::{
    db::Connection,
    io::{AsyncReader, AsyncWriter, CopyStreamError, copy_stream_with_hook},
    upload_file::UploadLocationResolver,
};
use uuid::Uuid;

use crate::{
    dto::attachment_dto::res::AttachmentDto, error::CtxError,
    port::attachment_repository::AttachmentRepository,
};

pub struct AttachmentInteractor<Conn, R, FC, ULR> {
    conn: Conn,
    repo: R,
    factory: AttachmentFactory<FC>,
    upload_location_resolver: ULR,
}

impl<Conn, R, FC, ULR> AttachmentInteractor<Conn, R, FC, ULR>
where
    Conn: Connection,
    R: AttachmentRepository<Conn> + AttachmentRepository<<Conn as Connection>::Transaction>,
    FC: FileTypeChecker,
    ULR: UploadLocationResolver,
{
    pub fn new(
        conn: Conn,
        repo: R,
        factory: AttachmentFactory<FC>,
        upload_location_resolver: ULR,
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
        extension: Extension,
        mut async_reader: impl AsyncReader,
        mut async_writer: impl AsyncWriter,
    ) -> Result<AttachmentDto, CtxError> {
        let mut buf = [0u8; 4096];
        let mut magic_checker_buf = vec![0u8; FC::MAGIC_CHECKER_HEADER_SIZE];
        let mut magic_filled = 0usize; // 已收集的字节数

        let total = copy_stream_with_hook(
            &mut async_reader,
            &mut async_writer,
            &mut buf,
            |chunk, written| {
                // 未收满时，从本块继续收集 MIME 头
                let remaining = FC::MAGIC_CHECKER_HEADER_SIZE - magic_filled;
                let to_copy = chunk.len().min(remaining);
                magic_checker_buf[magic_filled..magic_filled + to_copy]
                    .copy_from_slice(&chunk[..to_copy]);
                magic_filled += to_copy;

                // 超限立即中止：返回 Err 时本块不会被写入
                if !FileSize::allows(written as i64, chunk.len() as i64) {
                    return Err(CtxError::PayloadTooLarge(format!(
                        "attachment size {} bytes exceeds max size of {} bytes",
                        written + chunk.len() as u64,
                        FileSize::MAX_SIZE
                    )));
                }
                Ok(())
            },
        )
        .await
        .map_err(|e| match e {
            CopyStreamError::Reader(e) => e.into(),
            CopyStreamError::Writer(e) => e.into(),
            CopyStreamError::Hook(e) => e,
        })?;

        // 文件太小，不足以检测 MIME
        if magic_filled < FC::MAGIC_CHECKER_HEADER_SIZE {
            return Err(CtxError::InvalidInput(
                "file too small for MIME detection".to_string(),
            ));
        }

        let file_size = FileSize::new(total as i64)?;
        let attachment = self.factory.create_attachment(
            attachment_id,
            &magic_checker_buf,
            &extension,
            file_size,
        )?;
        let attachment = self.repo.insert(&self.conn, attachment).await?;

        let file_name = attachment.file_name();
        let url = self.upload_location_resolver.url(&file_name);
        Ok(AttachmentDto {
            id: attachment.into_parts().0,
            file_name,
            url,
        })
    }
}
