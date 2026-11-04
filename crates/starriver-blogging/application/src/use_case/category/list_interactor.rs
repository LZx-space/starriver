use starriver_shared_base::cache::Cache;
use starriver_shared_base::db::Connection;

use crate::dto::category_dto::output::CategoryDetailDto;
use crate::error::CtxError;
use crate::port::category_cache::{CACHE_KEY_CATEGORY_LIST, CategoryListKey};
use crate::port::category_query::CategoryQuery;

pub struct ListInteractor<Conn, Q, C> {
    conn: Conn,
    query: Q,
    cache: C,
}

impl<Conn, Q, C> ListInteractor<Conn, Q, C>
where
    Conn: Connection,
    Q: CategoryQuery<Conn>,
    C: Cache<CategoryListKey, Vec<CategoryDetailDto>>,
{
    pub fn new(conn: Conn, query: Q, cache: C) -> Self {
        Self { conn, query, cache }
    }

    pub async fn list(&self) -> Result<Vec<CategoryDetailDto>, CtxError> {
        self.cache
            .try_get_with(CACHE_KEY_CATEGORY_LIST, async {
                self.query.list(&self.conn).await
            })
            .await
            .map_err(|e| CtxError::internal("list categories failed", e))
    }
}
