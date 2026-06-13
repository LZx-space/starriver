use sea_orm::ConnectionTrait;
use starriver_shared_base::error::QueryError;

use crate::dto::category_dto::res::CategoryDetailDto;

pub trait CategoryQuery {
    fn list_all<C: ConnectionTrait>(
        &self,
        conn: &C,
    ) -> impl Future<Output = Result<Vec<CategoryDetailDto>, QueryError>> + Send;
}
