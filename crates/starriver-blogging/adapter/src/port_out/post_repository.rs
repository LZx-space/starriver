use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use starriver_blogging_domain::post::{entity::Post, repository::PostRepository};
use starriver_shared_base::{error::RepositoryError, repository::Revision};
use starriver_shared_framework::error_mapping::db_error_2_repo_error;
use time::OffsetDateTime;

use crate::port_out::po::{
    post_attachment_po,
    post_po::{ActiveModel, Entity},
};

pub struct DefaultPostRepository {
    conn: DatabaseConnection,
}

impl DefaultPostRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl PostRepository for DefaultPostRepository {
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Post>, RepositoryError> {
        let results = Entity::find_by_id(id)
            .find_with_related(post_attachment_po::Entity)
            .all(&self.conn)
            .await
            .map_err(db_error_2_repo_error)?;
        let Some((post, attachments)) = results.into_iter().next() else {
            return Ok(None);
        };
        let attachments = attachments
            .into_iter()
            .map(|e| e.attachment_id)
            .collect::<Vec<_>>();
        Post::from_repo(
            id,
            post.title,
            post.content,
            post.state.into(),
            post.author_id,
            post.category_id,
            attachments,
            post.published_at,
        )
        .map(Some)
        .map_err(|e| RepositoryError::BadData(e.to_string()))
    }

    async fn add(&self, post: Post) -> Result<Post, RepositoryError> {
        let (id, title, content, state, author_id, category_id, attachments, published_at) =
            post.dissolve();
        // 插入 Post 实体
        let model = ActiveModel {
            id: Set(id),
            title: Set(title.to_string()),
            content: Set(content.to_string()),
            state: Set(state.into()),
            author_id: Set(author_id),
            category_id: Set(category_id),
            published_at: Set(published_at),
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: NotSet,
        }
        .insert(&self.conn)
        .await
        .map_err(db_error_2_repo_error)?;

        // 插入附件关联
        if !attachments.is_empty() {
            post_attachment_po::Entity::insert_many(attachments.iter().map(|att_id| {
                post_attachment_po::ActiveModel {
                    post_id: Set(id),
                    attachment_id: Set(*att_id),
                    created_at: Set(OffsetDateTime::now_utc()),
                    updated_at: Set(None),
                }
            }))
            .exec(&self.conn)
            .await
            .map_err(db_error_2_repo_error)?;
        }

        // 构建 Post 实体
        let post = Post::from_repo(
            model.id,
            model.title,
            model.content,
            model.state.into(),
            model.author_id,
            model.category_id,
            attachments,
            model.published_at,
        )
        .map_err(|e| RepositoryError::BadData(e.to_string()))?;
        Ok(post)
    }

    async fn delete_by_id(&self, id: uuid::Uuid) -> Result<bool, RepositoryError> {
        let not_zero = Entity::delete_by_id(id)
            .exec(&self.conn)
            .await
            .map_err(db_error_2_repo_error)?
            .rows_affected
            != 0;
        Ok(not_zero)
    }

    async fn update(&self, post: Revision<Post>) -> Result<Post, RepositoryError> {
        let (original, modified) = post.dissolve();
        let (id, title, content, state, author_id, category_id, _, published_at) =
            original.dissolve();
        let (
            _,
            new_title,
            new_content,
            new_state,
            new_author_id,
            new_category_id,
            new_attachments,
            new_published_at,
        ) = modified.dissolve();

        // 更新博客
        let mut title = Unchanged(title.to_string());
        title.set_if_not_equals(new_title.to_string());

        let mut content = Unchanged(content.to_string());
        content.set_if_not_equals(new_content.to_string());

        let mut state = Unchanged(state.into());
        state.set_if_not_equals(new_state.into());

        let mut author_id = Unchanged(author_id);
        author_id.set_if_not_equals(new_author_id);

        let mut category_id = Unchanged(category_id);
        category_id.set_if_not_equals(new_category_id);

        let mut published_at = Unchanged(published_at);
        published_at.set_if_not_equals(new_published_at);

        let model = ActiveModel {
            id: Unchanged(id),
            title,
            content,
            state,
            author_id,
            category_id,
            published_at,
            created_at: NotSet,
            updated_at: Set(Some(OffsetDateTime::now_utc())),
        };

        let updated = model
            .update(&self.conn)
            .await
            .map_err(db_error_2_repo_error)?;

        if !new_attachments.is_empty() {
            // 更新附件关联
            post_attachment_po::Entity::delete_many()
                .filter(post_attachment_po::Column::PostId.eq(id))
                .exec(&self.conn)
                .await
                .map_err(db_error_2_repo_error)?;
            // 插入新的附件关联
            post_attachment_po::Entity::insert_many(new_attachments.iter().map(|att_id| {
                post_attachment_po::ActiveModel {
                    post_id: Set(id),
                    attachment_id: Set(*att_id),
                    created_at: Set(OffsetDateTime::now_utc()),
                    updated_at: Set(None),
                }
            }))
            .exec(&self.conn)
            .await
            .map_err(db_error_2_repo_error)?;
        }

        let updated = Post::from_repo(
            updated.id,
            updated.title,
            updated.content,
            updated.state.into(),
            updated.author_id,
            updated.category_id,
            new_attachments,
            updated.published_at,
        )
        .map_err(|e| RepositoryError::BadData(e.to_string()))?;
        Ok(updated)
    }
}
