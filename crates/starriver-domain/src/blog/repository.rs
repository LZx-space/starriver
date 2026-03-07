use starriver_infrastructure::error::error::ApiError;
use uuid::Uuid;

use crate::blog::entity::Blog;

/// 仓库
pub trait BlogRepository {
    /// 按ID查找
    fn find_by_id(&self, id: Uuid) -> impl Future<Output = Result<Option<Blog>, ApiError>> + Send;

    /// 新增
    fn add(&self, blog: Blog) -> impl Future<Output = Result<Blog, ApiError>> + Send;

    /// 删除
    fn delete_by_id(&self, id: Uuid) -> impl Future<Output = Result<bool, ApiError>> + Send;

    /// 修改
    fn update(&self, blog: Blog) -> impl Future<Output = Result<Blog, ApiError>> + Send;
}
