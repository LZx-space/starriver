use crate::{
    port_in::state::{CACHE_KEY_CATEGORY_LIST, CatagoryListCache},
    port_out::persistence::po::category_po::{Column, Entity},
};
use sea_orm::{ConnectionTrait, EntityTrait, QueryOrder};
use starriver_blogging_application::{
    dto::category_dto::res::CategoryDetailDto, port::category_query::CategoryQuery,
};
use starriver_shared_base::error::QueryError;
pub struct DefaultCategoryQuery {
    cache: CatagoryListCache,
}

impl DefaultCategoryQuery {
    pub fn new(cache: CatagoryListCache) -> Self {
        Self { cache }
    }
}

impl CategoryQuery for DefaultCategoryQuery {
    async fn list_all<C: ConnectionTrait>(
        &self,
        conn: &C,
    ) -> Result<Vec<CategoryDetailDto>, QueryError> {
        self.cache
            .try_get_with(CACHE_KEY_CATEGORY_LIST, async {
                Entity::find()
                    .order_by_asc(Column::CreatedAt)
                    .all(conn)
                    .await
                    .map(|v| {
                        v.into_iter()
                            .map(|e| CategoryDetailDto {
                                id: e.id,
                                name: e.name,
                                created_at: e.created_at,
                                updated_at: e.updated_at,
                            })
                            .collect()
                    })
            })
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))
    }
}
