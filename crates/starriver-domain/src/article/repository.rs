use uuid::Uuid;

use crate::{article::entity::Article, common_error::RepositoryError, common_model::Revision};

/// 仓库
pub trait ArticleRepository {
    /// 按ID查找
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Article>, RepositoryError>> + Send;

    /// 新增
    fn add(
        &self,
        article: Article,
    ) -> impl Future<Output = Result<Article, RepositoryError>> + Send;

    /// 删除
    fn delete_by_id(&self, id: Uuid) -> impl Future<Output = Result<bool, RepositoryError>> + Send;

    /// 修改
    fn update(
        &self,
        article: Revision<Article>,
    ) -> impl Future<Output = Result<Article, RepositoryError>> + Send;
}
