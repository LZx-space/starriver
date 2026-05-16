use derive_getters::{Dissolve, Getters};

use uuid::Uuid;

use crate::user::value_object::{Email, Password, UserState, Username};

// -----Aggregate Root User------------------------------------------------------
/// The user aggregate. User is the aggregate root.
#[derive(Clone, Debug, Getters, Dissolve)]
pub struct User {
    id: Uuid,
    username: Username,
    password: Password,
    email: Email,
    state: UserState,
}

impl User {
    pub fn new(
        id: Uuid,
        username: Username,
        password: Password,
        email: Email,
        state: UserState,
    ) -> Self {
        Self {
            id,
            username,
            password,
            email,
            state,
        }
    }

    pub fn lock(&mut self) {
        self.state = UserState::Locked;
    }

    pub fn activate(&mut self) {
        self.state = UserState::Active;
    }
}
