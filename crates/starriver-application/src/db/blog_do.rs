use sea_orm::entity::prelude::*;
use sea_orm::{DeriveActiveEnum, EnumIter};
use time::OffsetDateTime;

use starriver_domain::blog::value_object::BlogState;

/// 博客
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(schema_name = "public", table_name = "blog")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub body: String,
    pub state: BlogStateDo,
    pub author_id: Uuid,
    pub create_at: OffsetDateTime,
    pub update_at: Option<OffsetDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}

// ---------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum BlogStateDo {
    #[sea_orm(num_value = 0)]
    #[default]
    Draft,
    #[sea_orm(num_value = 1)]
    Released,
}

impl From<BlogStateDo> for BlogState {
    fn from(value: BlogStateDo) -> Self {
        if value.eq(&BlogStateDo::Draft) {
            BlogState::Draft
        } else {
            BlogState::Released
        }
    }
}
