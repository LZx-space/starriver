use derive_getters::{Dissolve, Getters};
use uuid::Uuid;

use crate::shared_error::DomainError;

#[derive(Clone, Debug, Getters, Dissolve)]
pub struct Category {
    id: Uuid,
    name: String,
}

impl Category {
    pub fn new(name: String) -> Result<Self, DomainError> {
        if name.chars().count() > 10 {
            return Err(DomainError::PostCategoryTooLong(name));
        }
        Ok(Self {
            id: Uuid::now_v7(),
            name,
        })
    }

    pub fn update(&mut self, name: String) -> Result<(), DomainError> {
        if name.chars().count() > 10 {
            return Err(DomainError::PostCategoryTooLong(name));
        }
        self.name = name;
        Ok(())
    }
}
