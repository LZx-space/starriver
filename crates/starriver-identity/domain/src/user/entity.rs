use derive_getters::{Dissolve, Getters};

use uuid::Uuid;

use crate::user::value_object::{Email, HashedPassword, UserState, Username};

// -----Aggregate Root User------------------------------------------------------
/// The user aggregate. User is the aggregate root.
#[derive(Clone, Debug, Getters, Dissolve)]
pub struct User {
    id: Uuid,
    username: Username,
    password: HashedPassword,
    email: Email,
    state: UserState,
}

impl User {
    pub fn new(
        id: Uuid,
        username: Username,
        password: HashedPassword,
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

    pub fn from_repo(
        id: Uuid,
        username: String,
        password: String,
        email: String,
        state: UserState,
    ) -> Self {
        let username = Username::from_repo(username);
        let password = HashedPassword::from_repo(password);
        let email = Email::from_repo(email);
        Self {
            id,
            username,
            password,
            email,
            state,
        }
    }

    pub(crate) fn lock(&mut self) {
        self.state = UserState::Locked;
    }

    pub fn activate(&mut self) {
        self.state = UserState::Active;
    }
}
