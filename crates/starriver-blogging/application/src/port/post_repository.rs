use starriver_blogging_domain::post::entity::Post;
use starriver_shared_base::{
    error::RepositoryError,
    repository::{Executor, Revision},
};
use uuid::Uuid;

/// 仓库
pub trait PostRepository<T: Executor> {
    /// 按ID查找
    fn find_by_id(
        &self,
        conn: &T,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Post>, RepositoryError>> + Send;

    /// 新增
    fn add(
        &self,
        conn: &T,
        post: Post,
    ) -> impl Future<Output = Result<Post, RepositoryError>> + Send;

    /// 删除
    fn delete(
        &self,
        conn: &T,
        id: Uuid,
    ) -> impl Future<Output = Result<bool, RepositoryError>> + Send;

    /// 修改
    fn update(
        &self,
        conn: &T,
        post: Revision<Post>,
    ) -> impl Future<Output = Result<Post, RepositoryError>> + Send;
}
