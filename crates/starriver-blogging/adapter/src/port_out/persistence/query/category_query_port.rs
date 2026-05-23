use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use starriver_blogging_application::{
    dto::category_dto::res::CategoryDetailDto, port_out::category_query_port::CategoryQueryPort,
};
use starriver_shared_base::error::QueryError;
use starriver_shared_framework::error_mapping::db_2_query_error;

use crate::port_out::persistence::po::category_po::{Column, Entity};

pub struct DefaultCategoryQueryPort {
    conn: DatabaseConnection,
}

impl DefaultCategoryQueryPort {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl CategoryQueryPort for DefaultCategoryQueryPort {
    async fn list_all(&self) -> Result<Vec<CategoryDetailDto>, QueryError> {
        Entity::find()
            .order_by_asc(Column::CreatedAt)
            .all(&self.conn)
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
            .map_err(db_2_query_error)
    }
}
