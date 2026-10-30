use starriver_shared_base::{
    db::Executor,
    dto::{OffsetPage, OffsetPageQuery},
    error::QueryError,
};

use crate::dto::user_dto::output::UserDetailDto;

pub trait UserQuery<T: Executor> {
    /// 查询一页数据
    fn paginate(
        &self,
        conn: &T,
        q: OffsetPageQuery,
    ) -> impl Future<Output = Result<OffsetPage<UserDetailDto>, QueryError>> + Send;

    fn exists_by_email(
        &self,
        conn: &T,
        email: &str,
    ) -> impl Future<Output = Result<bool, QueryError>>;
}
