use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;

use crate::password_encoder::PasswordEncoder;
use crate::security_event::entity::SecurityEvent;
use crate::security_event::value_object::SecurityEventType;
use crate::shared_error::DomainError;
use crate::user::entity::User;
use crate::user::policy::BadPasswordPolicy;
use crate::user::value_object::UserState;

#[derive(Clone)]
pub struct AuthenticationService<PE> {
    policy: BadPasswordPolicy,
    encoder: Arc<PE>,
}

impl<PE> AuthenticationService<PE>
where
    PE: PasswordEncoder,
{
    pub fn new(policy: BadPasswordPolicy, encoder: Arc<PE>) -> Self {
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
