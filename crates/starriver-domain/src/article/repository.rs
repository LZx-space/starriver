use starriver_infrastructure::error::ApiError;
use uuid::Uuid;

use crate::article::entity::Article;

/// 仓库
pub trait ArticleRepository {
    /// 按ID查找
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Article>, ApiError>> + Send;

    /// 新增
    fn add(&self, article: Article) -> impl Future<Output = Result<Article, ApiError>> + Send;

    /// 删除
    fn delete_by_id(&self, id: Uuid) -> impl Future<Output = Result<bool, ApiError>> + Send;

    /// 修改
    fn update(&self, article: Article) -> impl Future<Output = Result<Article, ApiError>> + Send;
}
