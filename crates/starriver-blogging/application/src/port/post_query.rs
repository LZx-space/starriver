use starriver_shared_base::{dto::PageResult, error::QueryError, repository::Executor};
use uuid::Uuid;

use crate::dto::post_dto::{
    req::PageQuery,
    res::{PostDetailDto, PostExcerptDto},
};

pub trait PostQuery<T: Executor> {
    /// 查询一页数据
    fn paginate(
        &self,
        conn: &T,
        q: PageQuery,
    ) -> impl Future<Output = Result<PageResult<PostExcerptDto>, QueryError>> + Send;

    fn find_detail(
        &self,
        conn: &T,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<PostDetailDto>, QueryError>> + Send;
}
