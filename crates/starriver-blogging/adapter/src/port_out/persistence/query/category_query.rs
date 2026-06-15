use crate::port_out::persistence::po::category_po::{Column, Entity};
use sea_orm::{EntityTrait, QueryOrder};
use starriver_blogging_application::{
    dto::category_dto::res::CategoryDetailDto, port::category_query::CategoryQuery,
};
use starriver_shared_base::error::QueryError;
use starriver_shared_framework::db::DefaultConnection;

pub struct DefaultCategoryQuery;

impl CategoryQuery<DefaultConnection> for DefaultCategoryQuery {
    async fn list_all(
        &self,
        conn: &DefaultConnection,
    ) -> Result<Vec<CategoryDetailDto>, QueryError> {
        Entity::find()
            .order_by_asc(Column::CreatedAt)
            .all(conn)
            .await
            .map_err(|e| QueryError::DbError(e.to_string()))
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
    }
}
