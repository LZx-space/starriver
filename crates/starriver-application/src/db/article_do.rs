use sea_orm::entity::prelude::*;
use sea_orm::{DeriveActiveEnum, EnumIter};
use time::OffsetDateTime;

use starriver_domain::article::value_object::ArticleState;

/// 博客
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "article")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub state: ArticleStateDo,
    pub author_id: Uuid,
    pub create_at: OffsetDateTime,
    pub update_at: Option<OffsetDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// ---------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum ArticleStateDo {
    #[sea_orm(num_value = 0)]
    #[default]
    Draft,
    #[sea_orm(num_value = 1)]
    Released,
}

impl From<ArticleStateDo> for ArticleState {
    fn from(value: ArticleStateDo) -> Self {
        if value.eq(&ArticleStateDo::Draft) {
            ArticleState::Draft
        } else {
            ArticleState::Published
        }
    }
}

impl From<ArticleState> for ArticleStateDo {
    fn from(value: ArticleState) -> Self {
        if value.eq(&ArticleState::Draft) {
            ArticleStateDo::Draft
        } else {
            ArticleStateDo::Released
        }
    }
}
