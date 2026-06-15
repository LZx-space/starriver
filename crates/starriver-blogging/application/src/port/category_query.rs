use starriver_shared_base::{db::Executor, error::QueryError};

use crate::dto::category_dto::res::CategoryDetailDto;

pub trait CategoryQuery<T: Executor> {
    fn list_all(
        &self,
        conn: &T,
    ) -> impl Future<Output = Result<Vec<CategoryDetailDto>, QueryError>> + Send;
}
