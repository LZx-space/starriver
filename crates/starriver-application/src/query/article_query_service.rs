use crate::{
    article_dto::{
        req::PageQuery,
        res::{ArticleAttachmentRow, ArticleDetail},
    },
    db::{
        article_attachment_do,
        article_do::{ArticleStateDo, Column, Entity, Relation},
        category_do,
    },
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, PaginatorTrait, QueryFilter,
    QuerySelect, RelationTrait,
};
use starriver_infrastructure::{
    error::ApiError,
    model::page::PageResult,
    util::html_utils::{DefaultExcerptor, Excerptor},
};
use uuid::Uuid;

use crate::article_dto::res::ArticleExcerpt;

pub trait ArticleQueryService {
    /// 查询一页数据
    fn paginate(
        &self,
        query: PageQuery,
    ) -> impl Future<Output = Result<PageResult<ArticleExcerpt>, ApiError>> + Send;

    fn find_detail(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<ArticleDetail>, ApiError>> + Send;
}

pub struct DefaultArticleQueryService {
    pub conn: DatabaseConnection,
}

impl ArticleQueryService for DefaultArticleQueryService {
    async fn paginate(&self, q: PageQuery) -> Result<PageResult<ArticleExcerpt>, ApiError> {
        let mut cond = Condition::all();
        if q.published_only {
            cond = cond.add(Column::State.eq(ArticleStateDo::Published));
        }
        if let Some(category_id) = q.category_id {
            cond = cond.add(Column::CategoryId.eq(category_id));
        }
        let articles = Entity::find()
            .select_only()
            .columns([
                Column::Id,
                Column::Title,
                Column::Content,
                Column::State,
                Column::PublishedAt,
                Column::CreatedAt,
                Column::UpdatedAt,
            ])
            .join(JoinType::LeftJoin, Relation::Category.def())
            .column_as(category_do::Column::Name, "category")
            .filter(cond.clone())
            .offset(q.page * q.page_size)
            .limit(q.page_size)
            .into_model::<ArticleExcerpt>()
            .all(&self.conn)
            .await
            .map_err(ApiError::from)?
            .into_iter()
            .map(|mut e| {
                e.excerpt = DefaultExcerptor::excerpt(&e.excerpt, 200);
                e
            })
            .collect::<Vec<_>>();
        let record_total = Entity::find()
            .select_only()
            .column(Column::Id)
            .filter(cond)
            .count(&self.conn)
            .await
            .map_err(ApiError::from)?;
        Ok(PageResult::new(q.page, q.page_size, record_total, articles))
    }

    async fn find_detail(&self, id: Uuid) -> Result<Option<ArticleDetail>, ApiError> {
        let article = Entity::find_by_id(id)
            .select_only()
            .columns([
                Column::Id,
                Column::Title,
                Column::Content,
                Column::State,
                Column::PublishedAt,
                Column::CreatedAt,
                Column::UpdatedAt,
            ])
            .column_as(Column::Id, "article_id")
            .join(JoinType::LeftJoin, Relation::Category.def())
            .columns([category_do::Column::Id, category_do::Column::Name])
            .into_model::<ArticleDetail>()
            .one(&self.conn)
            .await
            .map_err(ApiError::from)?;
        if let Some(mut article) = article {
            let attachments = article_attachment_do::Entity::find()
                .columns([
                    article_attachment_do::Column::Id,
                    article_attachment_do::Column::FileName,
                ])
                .filter(article_attachment_do::Column::ArticleId.eq(id))
                .into_model::<ArticleAttachmentRow>()
                .all(&self.conn)
                .await?;
            article.attachment_rows = attachments;
            return Ok(Some(article));
        }
        Ok(article)
    }
}
