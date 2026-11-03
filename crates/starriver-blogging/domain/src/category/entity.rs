use ammonia::clean;
use derive_getters::{Dissolve, Getters};
use serde::Serialize;
use uuid::Uuid;

use crate::shared_error::DomainError;

#[derive(Clone, Debug, Getters, Dissolve, PartialEq, Serialize)]
#[dissolve(rename = "into_parts")]
pub struct Category {
    id: Uuid,
    name: String,
}

impl Category {
    const MAX_LENGTH: usize = 10; // same with table ddl

    pub fn new(name: String) -> Result<Self, DomainError> {
        let name = validate_name(name)?;
        Ok(Self {
            id: Uuid::now_v7(),
            name,
        })
    }

    pub fn from_persisted(id: Uuid, name: String) -> Self {
        Self { id, name }
    }

    pub fn update(&mut self, name: String) -> Result<(), DomainError> {
        let name = validate_name(name)?;
        self.name = name;
        Ok(())
    }
}

fn validate_name(name: String) -> Result<String, DomainError> {
    if name.is_empty() {
        return Err(DomainError::PostCategoryIsNone);
    }
    let name = clean(&name);
    if name.chars().count() > Category::MAX_LENGTH {
        return Err(DomainError::PostCategoryTooLong(name));
    }
    Ok(name)
}
