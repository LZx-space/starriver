use derive_getters::{Dissolve, Getters};
use serde::Serialize;
use starriver_infrastructure::error::ApiError;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Getters, Dissolve, Serialize)]
pub struct Category {
    id: Uuid,
    name: String,
    created_at: OffsetDateTime,
    updated_at: Option<OffsetDateTime>,
}

impl Category {
    pub fn new(name: String) -> Result<Self, ApiError> {
        let len = name.len();
        if len > 10 {
            return Err(ApiError::with_bad_request("类别名称过长"));
        }
        Ok(Self {
            id: Uuid::now_v7(),
            name,
            created_at: OffsetDateTime::now_utc(),
            updated_at: None,
        })
    }

    pub fn from_repo(
        id: Uuid,
        name: String,
        created_at: OffsetDateTime,
        updated_at: Option<OffsetDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            created_at,
            updated_at,
        }
    }

    pub fn update(&mut self, name: String) -> Result<(), ApiError> {
        let len = name.len();
        if len > 10 {
            return Err(ApiError::with_bad_request("类别名称过长"));
        }
        self.name = name;
        self.updated_at = Some(OffsetDateTime::now_utc());
        Ok(())
    }
}
