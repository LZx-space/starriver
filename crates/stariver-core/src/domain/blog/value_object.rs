use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum State {
    Draft,
    Released,
}

impl Default for State {
    fn default() -> Self {
        State::Draft
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Draft => f.write_str("draft"),
            State::Released => f.write_str("released"),
        }
    }
}