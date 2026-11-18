use crate::port_out::persistence::po::category_po::{Column, Entity};
use sea_orm::{EntityTrait, QueryOrder};
use starriver_blogging_application::{
    dto::category_dto::output::CategoryDetailDto, port::category_query::CategoryQuery,
};
use starriver_shared_base::error::RepositoryError;
use starriver_shared_infra::{db::DefaultConnection, error_mapping::db_2_repo_error};

pub struct DefaultCategoryQuery;

impl CategoryQuery<DefaultConnection> for DefaultCategoryQuery {
    async fn list(
        &self,
        conn: &DefaultConnection,
    ) -> Result<Vec<CategoryDetailDto>, RepositoryError> {
        Entity::find()
            .order_by_asc(Column::CreatedAt)
            .all(conn)
            .await
            .map_err(db_2_repo_error)
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
