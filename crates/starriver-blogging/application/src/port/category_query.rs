use starriver_shared_base::{error::QueryError, repository::Executor};

use crate::dto::category_dto::res::CategoryDetailDto;

pub trait CategoryQuery<T: Executor> {
    fn list_all(
        &self,
        conn: &T,
    ) -> impl Future<Output = Result<Vec<CategoryDetailDto>, QueryError>> + Send;
}
