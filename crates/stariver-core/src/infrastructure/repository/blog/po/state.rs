use sea_orm::{DeriveActiveEnum, EnumIter};

use crate::domain::blog::value_object::State;

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
        ArticleState::Draft
    }
}

impl Into<State> for ArticleState {
    fn into(self) -> State {
        if self.eq(&ArticleState::Draft) {
            State::Draft
        } else {
            State::Released
        }
    }
}

impl From<State> for ArticleState {
    fn from(value: State) -> Self {
        if value.eq(&State::Draft) {
            ArticleState::Draft
        } else {
            ArticleState::Released
        }
    }
}
