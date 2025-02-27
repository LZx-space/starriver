use std::fmt::{Display, Formatter, Result};

use serde::Serialize;

#[derive(Default, Debug, Eq, PartialEq, Serialize)]
pub enum State {
    #[default]
    Draft,
    Released,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            State::Draft => f.write_str("draft"),
            State::Released => f.write_str("released"),
        }
    }
}
