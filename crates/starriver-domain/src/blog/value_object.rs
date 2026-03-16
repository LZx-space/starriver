use std::fmt::{Display, Formatter, Result};

use serde::Serialize;

#[derive(Default, Debug, Eq, PartialEq, Serialize)]
pub enum BlogState {
    #[default]
    Draft,
    Released,
}

impl Display for BlogState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BlogState::Draft => f.write_str("draft"),
            BlogState::Released => f.write_str("released"),
        }
    }
}
