use starriver_shared_base::error::RepositoryError;
use uuid::Uuid;

use crate::attachment::entity::Attachment;

pub trait AttachmentRepository {
    fn insert(
        &self,
        attachment: Attachment,
    ) -> impl Future<Output = Result<Attachment, RepositoryError>> + Send;

    fn delete(&self, id: Uuid) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
