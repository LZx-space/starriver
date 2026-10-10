use crate::authentication::core::principal::Principal;

pub struct AuthenticationResult<P: Principal> {
    principal: P,
}

impl<P: Principal> AuthenticationResult<P> {
    pub fn new(principal: P) -> Self {
        Self { principal }
    }

    pub fn principal(&self) -> &P {
        &self.principal
    }
}
