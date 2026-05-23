use starriver_shared_base::error::QueryError;

use crate::dto::category_dto::res::CategoryDetailDto;

pub trait CategoryQueryPort {
    fn list_all(&self) -> impl Future<Output = Result<Vec<CategoryDetailDto>, QueryError>> + Send;
}
