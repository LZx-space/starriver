use starriver_shared_base::{db::Executor, error::RepositoryError};

use crate::dto::category_dto::output::CategoryDetailDto;

pub trait CategoryQuery<T: Executor> {
    fn list(
        &self,
        conn: &T,
    ) -> impl Future<Output = Result<Vec<CategoryDetailDto>, RepositoryError>> + Send;
}
