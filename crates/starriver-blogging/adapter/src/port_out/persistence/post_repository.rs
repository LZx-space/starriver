use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
};
use starriver_blogging_application::port::post_repository::PostRepository;
use starriver_blogging_domain::post::entity::Post;
use starriver_shared_base::{db::Revision, error::RepositoryError};
use starriver_shared_framework::{
    db::{DefaultConnection, DefaultTransaction},
    error_mapping::db_2_repo_error,
};
use time::OffsetDateTime;

use crate::port_out::persistence::po::{
    post_attachment_po,
    post_po::{ActiveModel, Entity},
};

pub struct DefaultPostRepository;

impl DefaultPostRepository {
    async fn find_by_id(
        &self,
        conn: &impl ConnectionTrait,
        id: uuid::Uuid,
    ) -> Result<Option<Post>, RepositoryError> {
        let results = Entity::find_by_id(id)
            .find_with_related(post_attachment_po::Entity)
            .all(conn)
            .await
            .map_err(db_2_repo_error)?;
        let Some((post, attachments)) = results.into_iter().next() else {
            return Ok(None);
        };
        let attachments = attachments
            .into_iter()
            .map(|e| e.attachment_id)
            .collect::<Vec<_>>();
        Ok(Some(Post::from_repo(
            id,
            post.title,
            post.content,
            post.state.into(),
            post.author_id,
            post.category_id,
            attachments,
            post.published_at,
        )))
    }

    async fn add(&self, conn: &impl ConnectionTrait, post: Post) -> Result<Post, RepositoryError> {
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
        .insert(conn)
        .await
        .map_err(db_2_repo_error)?;

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
            .exec(conn)
            .await
            .map_err(db_2_repo_error)?;
        }

        // 构建 Post 实体
        Ok(Post::from_repo(
            model.id,
            model.title,
            model.content,
            model.state.into(),
            model.author_id,
            model.category_id,
            attachments,
            model.published_at,
        ))
    }

    async fn delete(
        &self,
        conn: &impl ConnectionTrait,
        id: uuid::Uuid,
    ) -> Result<bool, RepositoryError> {
        Entity::delete_by_id(id)
            .exec(conn)
            .await
            .map(|r| r.rows_affected != 0)
            .map_err(db_2_repo_error)
    }

    async fn update(
        &self,
        conn: &impl ConnectionTrait,
        post: Revision<Post>,
    ) -> Result<Post, RepositoryError> {
        let (original, modified) = post.dissolve();
        let (id, title, content, state, author_id, category_id, old_attachments, published_at) =
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

        let updated = ActiveModel {
            id: Unchanged(id),
            title,
            content,
            state,
            author_id,
            category_id,
            published_at,
            created_at: NotSet,
            updated_at: Set(Some(OffsetDateTime::now_utc())),
        }
        .update(conn)
        .await
        .map_err(db_2_repo_error)?;

        // 增量更新附件关联：只删移除的、只插新增的
        let to_insert: Vec<_> = new_attachments
            .iter()
            .filter(|a| !old_attachments.contains(a))
            .copied()
            .collect();
        let to_delete: Vec<_> = old_attachments
            .iter()
            .filter(|a| !new_attachments.contains(a))
            .copied()
            .collect();

        if !to_delete.is_empty() {
            post_attachment_po::Entity::delete_many()
                .filter(post_attachment_po::Column::PostId.eq(id))
                .filter(post_attachment_po::Column::AttachmentId.is_in(to_delete))
                .exec(conn)
                .await
                .map_err(db_2_repo_error)?;
        }
        if !to_insert.is_empty() {
            post_attachment_po::Entity::insert_many(to_insert.iter().map(|att_id| {
                post_attachment_po::ActiveModel {
                    post_id: Set(id),
                    attachment_id: Set(*att_id),
                    created_at: Set(OffsetDateTime::now_utc()),
                    updated_at: Set(None),
                }
            }))
            .exec(conn)
            .await
            .map_err(db_2_repo_error)?;
        }

        Ok(Post::from_repo(
            updated.id,
            updated.title,
            updated.content,
            updated.state.into(),
            updated.author_id,
            updated.category_id,
            new_attachments,
            updated.published_at,
        ))
    }
}

impl PostRepository<DefaultConnection> for DefaultPostRepository {
    async fn find_by_id(
        &self,
        conn: &DefaultConnection,
        id: uuid::Uuid,
    ) -> Result<Option<Post>, RepositoryError> {
        self.find_by_id(conn, id).await
    }

    async fn add(&self, conn: &DefaultConnection, post: Post) -> Result<Post, RepositoryError> {
        self.add(conn, post).await
    }

    async fn delete(
        &self,
        conn: &DefaultConnection,
        id: uuid::Uuid,
    ) -> Result<bool, RepositoryError> {
        self.delete(conn, id).await
    }

    async fn update(
        &self,
        conn: &DefaultConnection,
        post: Revision<Post>,
    ) -> Result<Post, RepositoryError> {
        self.update(conn, post).await
    }
}

impl PostRepository<DefaultTransaction> for DefaultPostRepository {
    async fn find_by_id(
        &self,
        conn: &DefaultTransaction,
        id: uuid::Uuid,
    ) -> Result<Option<Post>, RepositoryError> {
        self.find_by_id(conn, id).await
    }

    async fn add(&self, conn: &DefaultTransaction, post: Post) -> Result<Post, RepositoryError> {
        self.add(conn, post).await
    }

    async fn delete(
        &self,
        conn: &DefaultTransaction,
        id: uuid::Uuid,
    ) -> Result<bool, RepositoryError> {
        self.delete(conn, id).await
    }

    async fn update(
        &self,
        conn: &DefaultTransaction,
        post: Revision<Post>,
    ) -> Result<Post, RepositoryError> {
        self.update(conn, post).await
    }
}
