use std::collections::HashSet;

use crate::db::blog_attachment_do;
use crate::db::blog_attachment_do::Column;
use crate::db::blog_do::ActiveModel;
use crate::db::blog_do::Entity;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveValue::Unchanged;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait};
use starriver_domain::blog::entity::Attachment;
use starriver_domain::blog::entity::Blog;
use starriver_domain::blog::repository::BlogRepository;
use starriver_domain::blog::value_object::Content;
use starriver_domain::blog::value_object::Title;
use starriver_infrastructure::error::ApiError;
use starriver_infrastructure::error::Cause;
use starriver_infrastructure::util::db::TransactionalConn;
use time::OffsetDateTime;
use tracing::info;
use uuid::Uuid;

pub struct DefaultBlogRepository<T> {
    conn: T,
}

impl<T> DefaultBlogRepository<T>
where
    T: TransactionalConn,
{
    pub fn new(conn: T) -> DefaultBlogRepository<T> {
        Self { conn }
    }

    pub fn conn(self) -> T {
        self.conn
    }
}

impl<T> BlogRepository for DefaultBlogRepository<T>
where
    T: TransactionalConn,
{
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Blog>, ApiError> {
        find_by_id(&self.conn, id).await
    }

    async fn add(&self, blog: Blog) -> Result<Blog, ApiError> {
        let (id, title, content, state, _, author_id, _, _) = blog.dissolve();
        ActiveModel {
            id: Set(id),
            title: Set(title.to_string()),
            content: Set(content.to_string()),
            state: Set(state.into()),
            author_id: Set(author_id),
            create_at: Set(OffsetDateTime::now_utc()),
            update_at: NotSet,
        }
        .insert(&self.conn)
        .await
        .map(|e| {
            Blog::from_repo(
                e.id,
                Title::new(e.title).expect("never happens"),
                Content::new(e.content).expect("never happens"),
                e.state.into(),
                Vec::new(),
                e.author_id,
                e.create_at,
                e.update_at,
            )
        })
        .map_err(ApiError::from)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<bool, ApiError> {
        let result = Entity::delete_by_id(id)
            .exec(&self.conn)
            .await?
            .rows_affected
            > 0;
        Ok(result)
    }

    async fn update(&self, blog: Blog) -> Result<Blog, ApiError> {
        let (id, new_title, new_content, new_state, new_attachments, new_author_id, _, _) =
            blog.dissolve();
        match find_by_id(&self.conn, id).await? {
            Some(found) => {
                let (_, title, content, state, attachments, author_id, create_at, _) =
                    found.dissolve();

                let (to_delete, to_insert) = diff_attachments(&attachments, new_attachments);
                // 删除旧附件
                let to_delete_count = to_delete.len();
                info!("需要删除附件个数[{}]", to_delete_count);
                if to_delete_count > 0 {
                    blog_attachment_do::Entity::delete_many()
                        .filter(Column::Id.is_in(to_delete))
                        .exec(&self.conn)
                        .await?;
                }
                // 插入新附件
                let to_insert_count = to_insert.len();
                info!("需要新增附件个数[{}]", to_insert_count);
                if to_insert_count > 0 {
                    blog_attachment_do::Entity::insert_many(to_insert.into_iter())
                        .exec(&self.conn)
                        .await?;
                }

                // 更新博客
                let mut title = Unchanged(title.to_string());
                title.set_if_not_equals(new_title.to_string());

                let mut content = Unchanged(content.to_string());
                content.set_if_not_equals(new_content.to_string());

                let mut state = Unchanged(state.into());
                state.set_if_not_equals(new_state.into());

                let mut author_id = Unchanged(author_id);
                author_id.set_if_not_equals(new_author_id);

                let model = ActiveModel {
                    id: Unchanged(id),
                    title,
                    content,
                    state,
                    author_id,
                    create_at: Unchanged(create_at),
                    update_at: Set(Some(OffsetDateTime::now_utc())),
                };

                model
                    .update(&self.conn)
                    .await
                    .map(|e| {
                        Blog::from_repo(
                            e.id,
                            Title::new(e.title).expect("never happens"),
                            Content::new(e.content).expect("never happens"),
                            e.state.into(),
                            Vec::new(),
                            e.author_id,
                            e.create_at,
                            e.update_at,
                        )
                    })
                    .map_err(ApiError::from)
            }
            None => Err(ApiError::new(Cause::ClientBadRequest, "Blog not found")),
        }
    }
}

async fn find_by_id(
    conn: &impl sea_orm::ConnectionTrait,
    id: Uuid,
) -> Result<Option<Blog>, ApiError> {
    let blog = Entity::find_by_id(id)
        .one(conn)
        .await
        .map(|op| {
            op.map(|e| {
                Blog::from_repo(
                    id,
                    Title::new(e.title).expect("never happens"),
                    Content::new(e.content).expect("never happens"),
                    e.state.into(),
                    Vec::new(),
                    e.author_id,
                    e.create_at,
                    e.update_at,
                )
            })
        })
        .map_err(ApiError::from)?;
    if let Some(mut blog) = blog {
        let attachments = blog_attachment_do::Entity::find()
            .filter(blog_attachment_do::Column::BlogId.eq(id))
            .all(conn)
            .await?;
        let mut attachments: Vec<Attachment> = attachments
            .into_iter()
            .map(|e| Attachment::from_repo(e.id, e.blog_id, e.create_at, e.update_at))
            .collect();
        blog.attachments().append(&mut attachments);
        return Ok(Some(blog));
    }
    Ok(blog)
}

//////////////////////////////////////////////

/// # return
/// * (to_delete, to_insert)
pub fn diff_attachments(
    old: &[Attachment],
    new: Vec<Attachment>, // 接收所有权，或使用 &[Attachment] 后内部克隆
) -> (Vec<Uuid>, Vec<blog_attachment_do::ActiveModel>) {
    let old_ids: HashSet<Uuid> = old.iter().map(|att| *att.id()).collect();

    let to_delete: Vec<Uuid> = old
        .iter()
        .filter(|att| !new.iter().any(|a| a.id() == att.id()))
        .map(|att| *att.id())
        .collect();

    let to_insert: Vec<blog_attachment_do::ActiveModel> = new
        .into_iter()
        .filter(|att| !old_ids.contains(att.id()))
        .map(|e| e.into())
        .collect();

    (to_delete, to_insert)
}
