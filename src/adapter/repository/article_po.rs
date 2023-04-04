use sea_orm::entity::prelude::*;

/// 文章
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "article")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub body: String,
    pub creator_id: String,

    pub create_time: DateTimeLocal,
    pub tag_ids: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
