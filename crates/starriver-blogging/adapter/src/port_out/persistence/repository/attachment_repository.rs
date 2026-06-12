use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use starriver_blogging_application::port::attachment_repository::AttachmentRepository;
use starriver_blogging_domain::attachment::entity::Attachment;
use starriver_shared_base::error::RepositoryError;
use starriver_shared_framework::error_mapping::db_2_repo_error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::attachment_po::{ActiveModel, Entity};

pub struct DefaultAttachmentRepository {
    conn: DatabaseConnection,
}

impl DefaultAttachmentRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl AttachmentRepository for DefaultAttachmentRepository {
    async fn insert(&self, attachment: Attachment) -> Result<Attachment, RepositoryError> {
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
        .insert(&self.conn)
        .await
        .map_err(db_2_repo_error)
        .map(|e| Attachment::from_repo(e.id, e.file_name, e.file_size))
    }

    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(id)
            .exec(&self.conn)
            .await
            .map(|r| r.rows_affected > 0)
            .map_err(db_2_repo_error)
    }
}
