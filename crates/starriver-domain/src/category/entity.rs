use derive_getters::{Dissolve, Getters};
use serde::Serialize;
use uuid::Uuid;

use crate::common_error::DomainError;

#[derive(Clone, Debug, Getters, Dissolve, Serialize)]
pub struct Category {
    id: Uuid,
    name: String,
}

impl Category {
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.chars().count() > 10 {
            return Err(DomainError::ArticleCategoryTooLong(name));
        }
        Ok(Self {
            id: Uuid::now_v7(),
            name,
        })
    }

    pub fn from_repo(id: Uuid, name: String) -> Self {
        Self { id, name }
    }

    pub fn update(&mut self, name: String) -> Result<(), DomainError> {
        if name.chars().count() > 10 {
            return Err(DomainError::ArticleCategoryTooLong(name));
        }
        self.name = name;
        Ok(())
    }
}
