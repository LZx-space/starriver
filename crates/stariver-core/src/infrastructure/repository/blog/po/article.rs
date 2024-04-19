use sea_orm::entity::prelude::*;

use crate::infrastructure::repository::blog::po::state::ArticleState;

/// 文章
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "article")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    pub title: String,

    #[sea_orm(column_type = "Text")]
    pub body: String,

    pub state: ArticleState,

    pub author_id: String,

    pub create_at: DateTimeLocal,

    pub update_at: Option<DateTimeLocal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
