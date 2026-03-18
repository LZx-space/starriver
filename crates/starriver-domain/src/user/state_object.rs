use uuid::Uuid;

use crate::user::{entity::SecurityEvent, value_object::Password};

pub struct ChangePwdState {
    pub pwd: Password,
}

#[derive(Debug)]
pub struct AuthByPwdState {
    pub user_id: Uuid,
    pub locked: bool,
    pub bad_pwd_event: Option<SecurityEvent>,
}
