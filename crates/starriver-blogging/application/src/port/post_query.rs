use starriver_shared_base::{db::Executor, dto::PageResult, error::QueryError};
use uuid::Uuid;

use crate::dto::post_dto::{
    req::PageQuery,
    res::{PostDetailDto, PostExcerptDto, PostSearchDto},
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

    fn search(
        &self,
        conn: &T,
        q: &str,
    ) -> impl Future<Output = Result<Vec<PostSearchDto>, QueryError>> + Send;
}
