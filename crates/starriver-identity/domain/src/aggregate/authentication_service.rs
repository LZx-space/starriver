use std::ops::Add;
use std::time::Duration;
use time::OffsetDateTime;

use crate::aggregate::security_event::SecurityEvent;
use crate::aggregate::security_event_value_object::SecurityEventType;
use crate::aggregate::user::User;
use crate::aggregate::user_policy::BadPassswordPolicy;
use crate::aggregate::user_value_object::UserState;
use crate::common::error::DomainError;
use crate::common::traits::PasswordEncoder;

#[derive(Clone)]
pub struct AuthenticationService<PE> {
    policy: BadPassswordPolicy,
    encoder: PE,
}

impl<PE> AuthenticationService<PE>
where
    PE: PasswordEncoder,
{
    pub fn new(policy: BadPassswordPolicy, encoder: PE) -> Self {
        Self { policy, encoder }
    }

    pub fn authenticate(&self, user: &User, raw_password: &str) -> Result<(), DomainError> {
        match user.state() {
            UserState::Active => {}
            UserState::Locked => return Err(DomainError::UserLocked),
            UserState::Disabled => return Err(DomainError::UserDisabled),
        };

        self.encoder
            .verify(raw_password, user.password().as_str())
            .map(|_| Ok(()))
            .map_err(|e| DomainError::PasswordVerificationFailed(e.to_string()))?
    }

    /// 依据尝试密码次数来锁定账户
    pub fn check_and_lock_user(&self, user: &mut User, security_events: &[SecurityEvent]) {
        let now = OffsetDateTime::now_utc();
        if security_events
            .iter()
            .filter(|e| {
                let event_type = e.event_type();
                let created_at = e.created_at();
                matches!(event_type, SecurityEventType::TryLoginWithBadPwd)
                    && created_at.add(Duration::from_mins(self.policy.window_minutes)) >= now
            })
            .count()
            > self.policy.max_attempts
        {
            user.lock();
        }
    }
}
