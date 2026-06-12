use sea_orm::ConnectionTrait;
use starriver_blogging_domain::post::entity::Post;
use starriver_shared_base::{error::RepositoryError, repository::Revision};
use uuid::Uuid;

/// 仓库
pub trait PostRepository {
    /// 按ID查找
    fn find_by_id<C: ConnectionTrait>(
        &self,
        conn: &C,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Post>, RepositoryError>> + Send;

    /// 新增
    fn add<C: ConnectionTrait>(
        &self,
        conn: &C,
        post: Post,
    ) -> impl Future<Output = Result<Post, RepositoryError>> + Send;

    /// 删除
    fn delete<C: ConnectionTrait>(
        &self,
        conn: &C,
        id: Uuid,
    ) -> impl Future<Output = Result<bool, RepositoryError>> + Send;

    /// 修改
    fn update<C: ConnectionTrait>(
        &self,
        conn: &C,
        post: Revision<Post>,
    ) -> impl Future<Output = Result<Post, RepositoryError>> + Send;
}
