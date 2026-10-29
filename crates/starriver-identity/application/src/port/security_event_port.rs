use std::convert::Infallible;

use starriver_shared_base::{
    db::Executor,
    dto::{PageQuery, PageResult},
    error::QueryError,
};

use crate::dto::user_dto::{req::SecurityEventCmd, res::SecurityEventDto};

/// SecurityEvent无业务性，仅保存查询，所以这里不是Repository，简单的面向数据
pub trait SecurityEventPort<T: Executor> {
    fn paginate(
        &self,
        conn: &T,
        q: PageQuery,
    ) -> impl Future<Output = Result<PageResult<SecurityEventDto>, QueryError>> + Send;

    fn insert(
        &self,
        conn: &T,
        event: SecurityEventCmd,
    ) -> impl Future<Output = Result<(), Infallible>> + Send;
}
