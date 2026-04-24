use std::fmt::{Display, Formatter};

use serde::Serialize;
use starriver_infrastructure::error::ApiError;

#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize)]
pub enum ArticleState {
    #[default]
    Draft,
    Published,
}

impl Display for ArticleState {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ArticleState::Draft => f.write_str("draft"),
            ArticleState::Published => f.write_str("published"),
        }
    }
}

//////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Content(pub(crate) String);

impl Content {
    pub fn new(value: String) -> Result<Self, ApiError> {
        if value.len() > 50000 {
            return Err(ApiError::with_bad_request("content too long"));
        }
        Ok(Self(value))
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
