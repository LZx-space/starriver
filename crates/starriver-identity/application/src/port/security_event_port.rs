use std::convert::Infallible;

use starriver_shared_base::{db::Executor, dto::CursorPage, error::QueryError};

use crate::dto::user_dto::{
    input::{SecurityEventCmd, SecurityEventQuery},
    output::SecurityEventDto,
};

/// SecurityEvent无业务性，仅保存查询，所以这里不是Repository，简单的面向数据
pub trait SecurityEventPort<T: Executor> {
    fn paginate(
        &self,
        conn: &T,
        query: SecurityEventQuery,
    ) -> impl Future<Output = Result<CursorPage<SecurityEventDto>, QueryError>> + Send;

    fn insert(
        &self,
        conn: &T,
        event: SecurityEventCmd,
    ) -> impl Future<Output = Result<(), Infallible>> + Send;
}
