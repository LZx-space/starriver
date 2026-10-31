use crate::authentication::core::principal::Principal;

pub struct AuthenticationResult<P: Principal> {
    principal: P,
}

impl<P: Principal> AuthenticationResult<P> {
    pub fn new(principal: P) -> Self {
        Self { principal }
    }

    /// 消费结果，取出 principal，避免调用方为了拿所有权而 clone 字段
    pub fn into_principal(self) -> P {
        self.principal
    }
}
