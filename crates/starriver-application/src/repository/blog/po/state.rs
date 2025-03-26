use sea_orm::{DeriveActiveEnum, EnumIter};

use starriver_domain::blog::value_object::State;

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
