use std::fmt::{Display, Formatter};

use serde::Serialize;
use starriver_infrastructure::error::ApiError;

#[derive(Default, Debug, Eq, PartialEq, Serialize)]
pub enum BlogState {
    #[default]
    Draft,
    Published,
    Archived,
}

impl Display for BlogState {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            BlogState::Draft => f.write_str("draft"),
            BlogState::Published => f.write_str("published"),
            BlogState::Archived => f.write_str("archived"),
        }
    }
}

//////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Title(pub(crate) String);

impl Title {
    pub fn new(value: String) -> Result<Self, ApiError> {
        if value.len() > 30 {
            return Err(ApiError::with_bad_request("title too long"));
        }
        Ok(Self(value))
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug)]
pub struct Content(pub(crate) String);

impl Content {
    pub fn new(value: String) -> Result<Self, ApiError> {
        if value.len() > 10000 {
            return Err(ApiError::with_bad_request("content too long"));
        }
        Ok(Self(value))
    }

    pub fn ant_xxs(&self) -> Self {
        Self(self.0.replace("<", ""))
    }

    pub fn word_count(&self) -> usize {
        self.0.len()
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}
