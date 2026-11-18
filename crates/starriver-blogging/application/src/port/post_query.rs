use starriver_shared_base::{db::Executor, dto::OffsetPage, error::RepositoryError};
use uuid::Uuid;

use crate::dto::post_dto::{
    input::{PageQuery, SearchQuery},
    output::{PostDetailDto, PostExcerptDto, PostSearchDto},
};

pub trait PostQuery<T: Executor> {
    /// 查询一页数据
    fn paginate(
        &self,
        conn: &T,
        q: PageQuery,
    ) -> impl Future<Output = Result<OffsetPage<PostExcerptDto>, RepositoryError>> + Send;

    fn find_detail(
        &self,
        conn: &T,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<PostDetailDto>, RepositoryError>> + Send;

    fn search(
        &self,
        conn: &T,
        q: SearchQuery,
    ) -> impl Future<Output = Result<OffsetPage<PostSearchDto>, RepositoryError>> + Send;
}
