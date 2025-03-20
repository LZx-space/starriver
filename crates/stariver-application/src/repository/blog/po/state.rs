use sea_orm::{DeriveActiveEnum, EnumIter};

use stariver_domain::blog::value_object::State;

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum ArticleState {
    #[sea_orm(num_value = 0)]
    #[default]
    Draft,
    #[sea_orm(num_value = 1)]
    Released,
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
