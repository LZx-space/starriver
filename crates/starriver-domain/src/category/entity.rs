use derive_getters::{Dissolve, Getters};
use serde::Serialize;
use starriver_infrastructure::error::ApiError;
use uuid::Uuid;

#[derive(Clone, Debug, Getters, Dissolve, Serialize)]
pub struct Category {
    id: Uuid,
    name: String,
}

impl Category {
    pub fn new(name: String) -> Result<Self, ApiError> {
        if name.chars().count() > 10 {
            return Err(ApiError::with_bad_request("类别名称过长"));
        }
        Ok(Self {
            id: Uuid::now_v7(),
            name,
        })
    }

    pub fn from_repo(id: Uuid, name: String) -> Self {
        Self { id, name }
    }

    pub fn update(&mut self, name: String) -> Result<(), ApiError> {
        if name.chars().count() > 10 {
            return Err(ApiError::with_bad_request("类别名称过长"));
        }
        self.name = name;
        Ok(())
    }
}
