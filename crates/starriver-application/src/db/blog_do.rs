use sea_orm::entity::prelude::*;
use sea_orm::{DeriveActiveEnum, EnumIter};
use time::OffsetDateTime;

use starriver_domain::blog::value_object::State;

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
    pub state: BlogState,
    pub author_id: Uuid,
    pub create_at: OffsetDateTime,
    pub update_at: Option<OffsetDateTime>,
}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// ---------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum BlogState {
    #[sea_orm(num_value = 0)]
    #[default]
    Draft,
    #[sea_orm(num_value = 1)]
    Released,
}

impl Into<State> for BlogState {
    fn into(self) -> State {
        if self.eq(&BlogState::Draft) {
            State::Draft
        } else {
            State::Released
        }
    }
}

impl From<State> for BlogState {
    fn from(value: State) -> Self {
        if value.eq(&State::Draft) {
            BlogState::Draft
        } else {
            BlogState::Released
        }
    }
}
