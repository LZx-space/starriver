use anyhow::Error;
use uuid::Uuid;

use crate::blog::aggregate::Article;
use stariver_infrastructure::model::blog::ArticleSummary;
use stariver_infrastructure::model::page::{PageQuery, PageResult};

/// 仓库
pub trait ArticleRepository {
    /// 查询一页数据
    fn find_page(
        &self,
        query: PageQuery,
    ) -> impl Future<Output = Result<PageResult<ArticleSummary>, Error>> + Send;

    /// 按ID查找
    fn find_by_id(&self, id: Uuid) -> impl Future<Output = Result<Option<Article>, Error>> + Send;

    /// 新增
    fn add(&self, e: Article) -> impl Future<Output = Result<Article, Error>> + Send;

    /// 删除
    fn delete_by_id(&self, id: Uuid) -> impl Future<Output = Result<bool, Error>> + Send;

    /// 修改
    fn update(&self, e: Article) -> impl Future<Output = Result<Option<Article>, Error>> + Send;
}
