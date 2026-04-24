use crate::{
    article_dto::req::PageQuery,
    db::article_do::{Column, Entity},
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect,
};
use starriver_infrastructure::{
    error::ApiError,
    model::page::PageResult,
    util::html_utils::{DefaultExcerptor, Excerptor},
};

use crate::article_dto::res::ArticleExcerpt;

pub trait ArticleQueryService {
    /// 查询一页数据
    fn find_page(
        &self,
        query: PageQuery,
    ) -> impl Future<Output = Result<PageResult<ArticleExcerpt>, ApiError>> + Send;
}

pub struct DefaultArticleQueryService {
    pub conn: DatabaseConnection,
}

impl ArticleQueryService for DefaultArticleQueryService {
    async fn find_page(&self, q: PageQuery) -> Result<PageResult<ArticleExcerpt>, ApiError> {
        let mut cond = Condition::all();
        if q.published_only {
            cond = cond.add(Column::State.eq(1));
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
            ])
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
}
