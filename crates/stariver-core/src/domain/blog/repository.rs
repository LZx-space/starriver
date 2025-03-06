use anyhow::Error;
use sea_orm::prelude::async_trait::async_trait;
use uuid::Uuid;

use crate::domain::blog::aggregate::Article;
use crate::infrastructure::model::blog::ArticleSummary;
use crate::infrastructure::model::page::{PageQuery, PageResult};

/// 仓库
#[async_trait]
pub trait ArticleRepository {
    /// 查询一页数据
    async fn find_page(&self, query: PageQuery) -> Result<PageResult<ArticleSummary>, Error>;

    /// 按ID查找
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Article>, Error>;

    /// 新增
    async fn add(&self, e: Article) -> Result<Article, Error>;

    /// 删除
    async fn delete_by_id(&self, id: Uuid) -> Result<bool, Error>;

    /// 修改
    async fn update(&self, e: Article) -> Result<Option<Article>, Error>;
}
