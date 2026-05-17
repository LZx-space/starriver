use starriver_blogging_domain::attachment::{
    entity::Attachment, factory::AttachmentFactory, file_type_checker::FileTypeChecker,
};
use starriver_shared_base::io::AsyncReader;

use crate::{dto::attachment_dto::req::UploadAttachmentCmd, error::CtxError};

pub struct AttachmentApplication<Q, R, C, A> {
    query: Q,
    repo: R,
    factory: AttachmentFactory<C>,
    async_reader: A,
}

impl<Q, R, C, A> AttachmentApplication<Q, R, C, A>
where
    C: FileTypeChecker,
    A: AsyncReader,
{
    pub fn new(query: Q, repo: R, factory: AttachmentFactory<C>, async_reader: A) -> Self {
        Self {
            query,
            repo,
            factory,
            async_reader,
        }
    }

    pub async fn upload(&self, cmd: UploadAttachmentCmd) -> Result<Attachment, CtxError> {
        todo!()
    }
}
