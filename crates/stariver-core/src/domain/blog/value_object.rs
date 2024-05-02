use std::fmt::Display;

use serde::Serialize;

use crate::domain::blog::value_object::State::Draft;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum State {
    Draft,
    Released,
}

impl Default for State {
    fn default() -> Self {
        Draft
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "aa".to_string())
    }
}
