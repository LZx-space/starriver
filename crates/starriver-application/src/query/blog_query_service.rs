use crate::db::blog_do::{Column, Entity};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect, prelude::Expr};
use starriver_infrastructure::{
    error::ApiError,
    model::page::{PageQuery, PageResult},
};

use crate::blog_dto::res::BlogSummary;

pub trait BlogQueryService {
    /// 查询一页数据
    fn find_page(
        &self,
        query: PageQuery,
    ) -> impl Future<Output = Result<PageResult<BlogSummary>, ApiError>> + Send;
}

pub struct DefaultBlogQueryService {
    pub conn: DatabaseConnection,
}

impl BlogQueryService for DefaultBlogQueryService {
    async fn find_page(&self, q: PageQuery) -> Result<PageResult<BlogSummary>, ApiError> {
        let blogs = Entity::find()
            .select_only()
            .columns([Column::Id, Column::Title, Column::CreateAt])
            .column_as(Expr::cust("SUBSTRING(body, 1, 100)"), Column::Body)
            .offset(q.page * q.page_size)
            .limit(q.page_size)
            .into_model::<BlogSummary>()
            .all(&self.conn)
            .await
            .map_err(ApiError::from)?;
        let record_total = Entity::find()
            .select_only()
            .column(Column::Id)
            .count(&self.conn)
            .await
            .map_err(ApiError::from)?;
        Ok(PageResult::new(q.page, q.page_size, record_total, blogs))
    }
}
