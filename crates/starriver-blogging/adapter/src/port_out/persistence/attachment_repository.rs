use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait, EntityTrait};
use starriver_blogging_application::port::attachment_repository::AttachmentRepository;
use starriver_blogging_domain::attachment::entity::Attachment;
use starriver_shared_base::error::RepositoryError;
use starriver_shared_framework::{
    db::{DefaultConnection, DefaultTransaction},
    error_mapping::db_2_repo_error,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::attachment_po::{ActiveModel, Entity};

pub struct DefaultAttachmentRepository;

impl DefaultAttachmentRepository {
    async fn insert(
        &self,
        conn: &impl ConnectionTrait,
        attachment: Attachment,
    ) -> Result<Attachment, RepositoryError> {
        let file_name = attachment.file_name();
        let file_size = attachment.file_size();
        let fields = attachment.dissolve();
        ActiveModel {
            id: Set(fields.0),
            file_name: Set(file_name),
            file_size: Set(file_size),
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: Set(None),
        }
        .insert(conn)
        .await
        .map_err(db_2_repo_error)
        .map(|e| Attachment::from_repo(e.id, e.file_name, e.file_size))
    }

    async fn delete(&self, conn: &impl ConnectionTrait, id: Uuid) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(id)
            .exec(conn)
            .await
            .map(|r| r.rows_affected > 0)
            .map_err(db_2_repo_error)
    }
}

impl AttachmentRepository<DefaultConnection> for DefaultAttachmentRepository {
    async fn insert(
        &self,
        conn: &DefaultConnection,
        attachment: Attachment,
    ) -> Result<Attachment, RepositoryError> {
        self.insert(conn, attachment).await
    }

    async fn delete(&self, conn: &DefaultConnection, id: Uuid) -> Result<bool, RepositoryError> {
        self.delete(conn, id).await
    }
}

impl AttachmentRepository<DefaultTransaction> for DefaultAttachmentRepository {
    async fn insert(
        &self,
        conn: &DefaultTransaction,
        attachment: Attachment,
    ) -> Result<Attachment, RepositoryError> {
        self.insert(conn, attachment).await
    }

    async fn delete(&self, conn: &DefaultTransaction, id: Uuid) -> Result<bool, RepositoryError> {
        self.delete(conn, id).await
    }
}
