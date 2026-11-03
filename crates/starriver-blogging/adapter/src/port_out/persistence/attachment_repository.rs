use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ConnectionTrait, EntityTrait,
};
use starriver_blogging_application::port::attachment_repository::AttachmentRepository;
use starriver_blogging_domain::attachment::{
    entity::Attachment,
    value_object::{Extension, FileSize},
};
use starriver_shared_base::{db::Executor, error::RepositoryError};
use starriver_shared_infra::error_mapping::db_2_repo_error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::attachment_po::{ActiveModel, Entity};

pub struct DefaultAttachmentRepository;

impl<T> AttachmentRepository<T> for DefaultAttachmentRepository
where
    T: Executor + ConnectionTrait,
{
    async fn insert(
        &self,
        conn: &T,
        attachment: Attachment,
    ) -> Result<Attachment, RepositoryError> {
        let file_name = attachment.file_name();
        let file_size = attachment.file_size();
        let (id, ..) = attachment.into_parts();
        ActiveModel {
            id: Set(id),
            file_name: Set(file_name),
            file_size: Set(file_size),
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: NotSet,
        }
        .insert(conn)
        .await
        .map_err(db_2_repo_error)
        .map(|e| {
            let extension = e
                .file_name
                .split(".")
                .last()
                .unwrap_or_default()
                .to_string();
            let extension = Extension::from_persisted(extension);
            let file_size = FileSize::from_persisted(e.file_size);
            Attachment::from_persisted(e.id, extension, file_size)
        })
    }

    async fn delete(&self, conn: &T, id: Uuid) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(id)
            .exec(conn)
            .await
            .map(|r| r.rows_affected > 0)
            .map_err(db_2_repo_error)
    }
}
