use std::fmt::{Display, Formatter};

use crate::shared_error::DomainError;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub enum PostState {
    #[default]
    Draft,
    Published,
}

impl Display for PostState {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            PostState::Draft => f.write_str("draft"),
            PostState::Published => f.write_str("published"),
        }
    }
}

//////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Title(pub(crate) String);

impl Title {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.chars().count() > 50 {
            return Err(DomainError::PostTitleTooLong(value));
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
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.chars().count() > 50000 {
            return Err(DomainError::PostContentTooLong(value));
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
