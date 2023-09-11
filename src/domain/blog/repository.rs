use crate::domain::blog::aggregate::Article;
use crate::infrastructure::model::blog::ArticleSummary;
use crate::infrastructure::model::page::{PageQuery, PageResult};
use sea_orm::error::DbErr;
use sea_orm::prelude::async_trait::async_trait;
use uuid::Uuid;

/// 仓库
#[async_trait]
pub trait ArticleRepository {
    /// 查询一页数据
    async fn find_page(&self, query: PageQuery) -> Result<PageResult<ArticleSummary>, DbErr>;

    /// 按ID查找
    async fn find_one(&self, id: Uuid) -> Result<Option<Article>, DbErr>;

    /// 新增
    async fn add(&self, e: Article) -> Result<bool, DbErr>;

    /// 删除
    async fn delete(&self, id: Uuid) -> Result<bool, DbErr>;

    /// 修改
    async fn update(&self, e: Article) -> Result<bool, DbErr>;
}
