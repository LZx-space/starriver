use starriver_infrastructure::error::error::AppError;
use uuid::Uuid;

use crate::blog::entity::Blog;
use starriver_infrastructure::model::blog::BlogPreview;
use starriver_infrastructure::model::page::{PageQuery, PageResult};

/// 仓库
pub trait BlogRepository {
    /// 查询一页数据
    fn find_page(
        &self,
        query: PageQuery,
    ) -> impl Future<Output = Result<PageResult<BlogPreview>, AppError>> + Send;

    /// 按ID查找
    fn find_by_id(&self, id: Uuid) -> impl Future<Output = Result<Option<Blog>, AppError>> + Send;

    /// 新增
    fn add(&self, e: Blog) -> impl Future<Output = Result<Blog, AppError>> + Send;

    /// 删除
    fn delete_by_id(&self, id: Uuid) -> impl Future<Output = Result<bool, AppError>> + Send;

    /// 修改
    fn update(&self, e: Blog) -> impl Future<Output = Result<Option<Blog>, AppError>> + Send;
}
