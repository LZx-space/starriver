use starriver_blogging_domain::attachment::entity::Attachment;
use starriver_shared_base::error::RepositoryError;
use uuid::Uuid;

pub trait AttachmentRepository {
    fn insert(
        &self,
        attachment: Attachment,
    ) -> impl Future<Output = Result<Attachment, RepositoryError>> + Send;

    fn delete(&self, id: Uuid) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
