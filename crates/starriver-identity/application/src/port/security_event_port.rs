use starriver_shared_base::{
    db::Executor,
    dto::{PageQuery, PageResult},
    error::{QueryError, RepositoryError},
};

use crate::dto::user_dto::{req::SecurityEventCmd, res::SecurityEventDto};

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
    ) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}
