use std::path::Path;

use starriver_blogging_domain::attachment::{
    entity::Attachment,
    factory::AttachmentFactory,
    file_type_checker::FileTypeChecker,
    value_object::{ClaimedExtension, FileSize},
};
use starriver_shared_base::{
    db::{Connection, Transaction},
    io::{
        CopyStreamError, FileAsyncWriter, FileWriterFactory, NamedAsyncReader,
        copy_stream_with_hook,
    },
    upload_file::UploadLocationResolver,
};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    dto::attachment_dto::output::AttachmentDto, error::CtxError,
    port::attachment_repository::AttachmentRepository,
};

pub struct AttachmentInteractor<Conn, R, FC, FWF, ULR> {
    conn: Conn,
    repo: R,
    attachment_factory: AttachmentFactory<FC>,
    file_writer_factory: FWF,
    upload_location_resolver: ULR,
}

impl<Conn, R, FC, FWF, ULR> AttachmentInteractor<Conn, R, FC, FWF, ULR>
where
    Conn: Connection,
    R: AttachmentRepository<Conn> + AttachmentRepository<<Conn as Connection>::Transaction>,
    FC: FileTypeChecker,
    FWF: FileWriterFactory,
    ULR: UploadLocationResolver,
{
    pub fn new(
        conn: Conn,
        repo: R,
        attachment_factory: AttachmentFactory<FC>,
        file_writer_factory: FWF,
        upload_location_resolver: ULR,
    ) -> Self {
        Self {
            conn,
            repo,
            attachment_factory,
            file_writer_factory,
            upload_location_resolver,
        }
    }

    pub async fn upload(
        &self,
        mut async_reader: impl NamedAsyncReader,
    ) -> Result<AttachmentDto, CtxError> {
        let file_name = async_reader.file_name();
        let file_claimed_ext = Path::new(&file_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        let claimed_extension = ClaimedExtension::new(file_claimed_ext).map_err(CtxError::from)?;
        info!(file_name = %file_name, "upload attachment");

        let tx = self.conn.begin().await.map_err(|e| {
            error!(error = %e, "begin transaction failed");
            CtxError::Internal
        })?;

        let temp_file_path = self
            .upload_location_resolver
            .temp_file_path(&Uuid::now_v7());
        info!(temp_file_path = ?temp_file_path, "upload attachment, create temp file");
        let mut async_writer = self.file_writer_factory.create(&temp_file_path).await?;

        let inner_result: Result<Attachment, CtxError> = async {
            let mut buf = [0u8; 8 * 1024];
            let mut magic_checker_buf = vec![0u8; FC::MAGIC_HEADER_MAX_BYTES];
            let mut magic_filled = 0usize; // 已收集的字节数
            let total = copy_stream_with_hook(
                &mut async_reader,
                &mut async_writer,
                &mut buf,
                |chunk, written| {
                    // 未收满时，从本块继续收集 MIME 头
                    if magic_filled < FC::MAGIC_HEADER_MAX_BYTES {
                        let remaining = FC::MAGIC_HEADER_MAX_BYTES - magic_filled;
                        let to_copy = chunk.len().min(remaining);
                        magic_checker_buf[magic_filled..magic_filled + to_copy]
                            .copy_from_slice(&chunk[..to_copy]);
                        magic_filled += to_copy;
                    }

                    // 超限立即中止：返回 Err 时本块不会被写入
                    if !FileSize::allows(written as i64, chunk.len() as i64) {
                        return Err(CtxError::PayloadTooLarge(format!(
                            "Attachment size {} bytes exceeds the maximum allowed size of {} bytes",
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

            info!(file_size = %total, "upload attachment, stream copied");
            let file_size = FileSize::new(total as i64)?;

            let attachment =
                self.attachment_factory
                    .create(&magic_checker_buf, claimed_extension, file_size)?;
            let attachment = self.repo.insert(&tx, attachment).await?;
            Ok(attachment)
        }
        .await;

        match inner_result {
            Ok(attachment) => {
                tx.commit().await.map_err(|e| {
                    error!(error = %e, "upload attachment, commit transaction failed");
                    CtxError::Internal
                })?;
                let file_name = attachment.file_name();
                let file_path = self.upload_location_resolver.file_path(&file_name);
                info!(
                    temp_file_path = ?temp_file_path,
                    target_file_path = ?file_path,
                    "upload attachment, renaming temp file to final location"
                );
                async_writer.rename(file_path).await.map_err(|e| {
                    error!(
                        error = %e,
                        attachment_id = %attachment.id(),
                        temp_file_path = ?temp_file_path,
                        "upload attachment, rename failed after commit; orphan temp file + db record"
                    );
                    e
                })?;
                let url = self.upload_location_resolver.url(&file_name);
                Ok(AttachmentDto {
                    id: *attachment.id(),
                    file_name,
                    url,
                })
            }
            Err(err) => {
                if let Err(cleanup_err) = async_writer.remove().await {
                    warn!(
                        error = %cleanup_err,
                        "failed to remove temp file after upload failure"
                    );
                }
                if let Err(rb_err) = tx.rollback().await {
                    error!(error = %rb_err, "rollback failed");
                }
                Err(err)
            }
        }
    }
}
