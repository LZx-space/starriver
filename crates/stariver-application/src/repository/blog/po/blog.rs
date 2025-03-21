use crate::repository::blog::po::state::BlogState;
use sea_orm::entity::prelude::*;
use time::OffsetDateTime;

/// 文章
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blog")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub title: String,

    #[sea_orm(column_type = "Text")]
    pub body: String,

    pub state: BlogState,

    pub blogger_id: String,

    pub create_at: OffsetDateTime,

    pub update_at: Option<OffsetDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
