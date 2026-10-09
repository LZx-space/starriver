use std::time::SystemTime;

use crate::authentication::core::principal::Principal;

pub struct AuthenticationResult<P: Principal> {
    principal: P,
    authenticate_at: SystemTime,
}

impl<P: Principal> AuthenticationResult<P> {
    pub fn new(principal: P) -> Self {
        Self {
            principal,
            authenticate_at: SystemTime::now(),
        }
    }

    pub fn principal(&self) -> &P {
        &self.principal
    }

    pub fn authenticate_at(&self) -> &SystemTime {
        &self.authenticate_at
    }
}
