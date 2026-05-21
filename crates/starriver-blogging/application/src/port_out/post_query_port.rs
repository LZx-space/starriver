use starriver_shared_base::{dto::PageResult, error::QueryError};
use uuid::Uuid;

use crate::dto::post_dto::{
    req::PageQuery,
    res::{PostDetailDto, PostExcerptDto},
};

pub trait PostQueryPort {
    /// 查询一页数据
    fn paginate(
        &self,
        q: PageQuery,
    ) -> impl Future<Output = Result<PageResult<PostExcerptDto>, QueryError>> + Send;

    fn find_detail(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<PostDetailDto>, QueryError>> + Send;
}
