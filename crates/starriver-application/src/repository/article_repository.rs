use crate::db::article_attachment_do;
use crate::db::article_attachment_do::Column;
use crate::db::article_do::ActiveModel;
use crate::db::article_do::Entity;
use sea_orm::ActiveValue;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveValue::Unchanged;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait};
use starriver_domain::article::entity::Article;
use starriver_domain::article::entity::Attachment;
use starriver_domain::article::repository::ArticleRepository;
use starriver_domain::article::value_object::Content;
use starriver_domain::article::value_object::Title;
use starriver_infrastructure::error::ApiError;
use starriver_infrastructure::model::aggregate_revision::Revision;
use starriver_infrastructure::util::db::TransactionalConn;
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

pub struct DefaultArticleRepository<T> {
    conn: T,
}

impl<T> DefaultArticleRepository<T>
where
    T: TransactionalConn,
{
    pub fn new(conn: T) -> DefaultArticleRepository<T> {
        Self { conn }
    }

    pub fn conn(self) -> T {
        self.conn
    }
}

impl<T> ArticleRepository for DefaultArticleRepository<T>
where
    T: TransactionalConn,
{
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, ApiError> {
        let article = Entity::find_by_id(id)
            .one(&self.conn)
            .await
            .map(|op| {
                op.map(|e| {
                    Article::from_repo(
                        id,
                        Title::new(e.title).expect("never happens"),
                        Content::new(e.content).expect("never happens"),
                        e.state.into(),
                        Vec::new(),
                        e.author_id,
                        e.category_id,
                        e.published_at,
                    )
                })
            })
            .map_err(ApiError::from)?;
        if let Some(mut article) = article {
            let attachments = article_attachment_do::Entity::find()
                .filter(article_attachment_do::Column::ArticleId.eq(id))
                .all(&self.conn)
                .await?;
            let mut attachments: Vec<Attachment> = attachments
                .into_iter()
                .map(|e| Attachment::from_repo(e.id, e.extension, e.article_id))
                .collect();
            article.attachments().append(&mut attachments);
            return Ok(Some(article));
        }
        Ok(article)
    }

    async fn add(&self, article: Article) -> Result<Article, ApiError> {
        let (id, title, content, state, _, author_id, category_id, _) = article.dissolve();
        ActiveModel {
            id: Set(id),
            title: Set(title.to_string()),
            content: Set(content.to_string()),
            state: Set(state.into()),
            author_id: Set(author_id),
            category_id: Set(category_id),
            published_at: NotSet,
            created_at: Set(OffsetDateTime::now_utc()),
            updated_at: NotSet,
        }
        .insert(&self.conn)
        .await
        .map(|e| {
            Article::from_repo(
                e.id,
                Title::new(e.title).expect("never happens"),
                Content::new(e.content).expect("never happens"),
                e.state.into(),
                Vec::new(),
                e.author_id,
                e.category_id,
                e.published_at,
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

    async fn update(&self, article: Revision<Article>) -> Result<Article, ApiError> {
        let (original, modified) = article.dissolve();
        let (id, title, content, state, attachments, author_id, category_id, published_at) =
            original.dissolve();
        let (
            _,
            new_title,
            new_content,
            new_state,
            new_attachments,
            new_author_id,
            new_category_id,
            new_published_at,
        ) = modified.dissolve();

        let (to_delete_ids, to_add_attachments) = diff_attachment(&attachments, new_attachments);

        // 删除不用的附件
        let to_delete_count = to_delete_ids.len();
        debug!("attachments to delete: {}", to_delete_count);
        if to_delete_count > 0 {
            article_attachment_do::Entity::delete_many()
                .filter(Column::Id.is_in(to_delete_ids))
                .exec(&self.conn)
                .await?;
        }

        // 新增附件
        let to_add_count = to_add_attachments.len();
        debug!("attachments to add: {}", to_add_count);
        if to_add_count > 0 {
            let models: Vec<article_attachment_do::ActiveModel> = to_add_attachments
                .into_iter()
                .map(|att| article_attachment_do::ActiveModel {
                    id: ActiveValue::Set(*att.id()),
                    extension: ActiveValue::Set(att.extension().clone()),
                    article_id: ActiveValue::Set(*att.article_id()),
                    created_at: ActiveValue::Set(OffsetDateTime::now_utc()),
                    updated_at: ActiveValue::Set(None),
                })
                .collect();
            article_attachment_do::Entity::insert_many(models)
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

        model
            .update(&self.conn)
            .await
            .map(|e| {
                Article::from_repo(
                    e.id,
                    Title::new(e.title).expect("never happens"),
                    Content::new(e.content).expect("never happens"),
                    e.state.into(),
                    Vec::new(),
                    e.author_id,
                    e.category_id,
                    e.published_at,
                )
            })
            .map_err(ApiError::from)
    }
}

//////////////////////////////////////////////

/// # return
/// (to_delete_ids, to_add_attachments)
pub fn diff_attachment(old: &[Attachment], new: Vec<Attachment>) -> (Vec<Uuid>, Vec<Attachment>) {
    let to_delete_ids: Vec<Uuid> = old
        .iter()
        .filter(|att| !new.iter().any(|a| a.id() == att.id()))
        .map(|att| *att.id())
        .collect();
    let to_add: Vec<Attachment> = new
        .into_iter()
        .filter(|att| !old.iter().any(|a| a.id() == att.id()))
        .collect();
    (to_delete_ids, to_add)
}
