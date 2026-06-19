use starriver_shared_base::{
    db::Executor,
    dto::{PageQuery, PageResult},
    error::QueryError,
};
use uuid::Uuid;

use crate::dto::user_dto::res::UserDetailDto;

pub trait UserQuery<T: Executor> {
    /// 查询一页数据
    fn paginate(
        &self,
        conn: &T,
        q: PageQuery,
    ) -> impl Future<Output = Result<PageResult<UserDetailDto>, QueryError>> + Send;

    fn exists_by_email(
        &self,
        conn: &T,
        email: &str,
    ) -> impl Future<Output = Result<bool, QueryError>>;

    fn find_email_by_user_id(
        &self,
        conn: &T,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Option<String>, QueryError>>;
}
