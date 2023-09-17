use crate::domain::blog::value_object::State::Draft;

#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Draft,
    Released,
}

impl Default for State {
    fn default() -> Self {
        Draft
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        "aa".to_string()
    }
}
