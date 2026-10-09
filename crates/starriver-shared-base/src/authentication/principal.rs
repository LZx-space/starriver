use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::authentication::core::principal::Principal;
/// todo, impl principal by config
#[derive(Deserialize, Serialize)]
pub struct DefaultUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

impl DefaultUser {
    pub fn new(id: Uuid, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
        }
    }
}

impl Principal for DefaultUser {
    type Id = String;

    fn id(&self) -> &Self::Id {
        &self.username
    }
}
