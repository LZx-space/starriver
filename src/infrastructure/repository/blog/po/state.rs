use crate::domain::blog::value_object::State;
use sea_orm::{DeriveActiveEnum, EnumIter};

use crate::infrastructure::repository::blog::po::state::ArticleState::Draft;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum ArticleState {
    #[sea_orm(num_value = 0)]
    Draft,
    #[sea_orm(num_value = 1)]
    Released,
}

impl Default for ArticleState {
    fn default() -> Self {
        Draft
    }
}

impl Into<State> for ArticleState {
    fn into(self) -> State {
        if self.eq(&Draft) {
            State::Draft
        } else {
            State::Released
        }
    }
}
