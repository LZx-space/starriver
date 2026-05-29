use starriver_shared_base::{error::RepositoryError, repository::Revision};
use uuid::Uuid;

use crate::post::entity::Post;

/// 仓库
pub trait PostRepository {
    /// 按ID查找
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Post>, RepositoryError>> + Send;

    /// 新增
    fn add(&self, post: Post) -> impl Future<Output = Result<Post, RepositoryError>> + Send;

    /// 删除
    fn delete(&self, id: Uuid) -> impl Future<Output = Result<bool, RepositoryError>> + Send;

    /// 修改
    fn update(
        &self,
        post: Revision<Post>,
    ) -> impl Future<Output = Result<Post, RepositoryError>> + Send;
}
