use starriver_blogging_domain::attachment::entity::Attachment;
use starriver_shared_base::{db::Executor, error::RepositoryError};
use uuid::Uuid;

pub trait AttachmentRepository<T: Executor> {
    fn insert(
        &self,
        conn: &T,
        attachment: Attachment,
    ) -> impl Future<Output = Result<Attachment, RepositoryError>> + Send;

    fn delete(
        &self,
        conn: &T,
        id: Uuid,
    ) -> impl Future<Output = Result<bool, RepositoryError>> + Send;
}
