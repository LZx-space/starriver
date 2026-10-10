use uuid::Uuid;

use crate::authentication::core::principal::Principal;
/// todo, impl principal by config
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
    type Id = Uuid;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
