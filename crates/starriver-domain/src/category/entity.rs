use derive_getters::{Dissolve, Getters};
use serde::Serialize;
use starriver_infrastructure::error::ApiError;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Getters, Dissolve, Serialize)]
pub struct Category {
    id: Uuid,
    name: String,
    create_at: OffsetDateTime,
    update_at: Option<OffsetDateTime>,
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
            create_at: OffsetDateTime::now_utc(),
            update_at: None,
        })
    }

    pub fn from_repo(
        id: Uuid,
        name: String,
        create_at: OffsetDateTime,
        update_at: Option<OffsetDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            create_at,
            update_at,
        }
    }

    pub fn update(&mut self, name: String) -> Result<(), ApiError> {
        let len = name.len();
        if len > 10 {
            return Err(ApiError::with_bad_request("类别名称过长"));
        }
        self.name = name;
        self.update_at = Some(OffsetDateTime::now_utc());
        Ok(())
    }
}
