use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;

use crate::error::DomainError;
use crate::password_encoder::PasswordEncoder;
use crate::security_event::entity::SecurityEvent;
use crate::security_event::value_object::SecurityEventType;
use crate::user::entity::User;
use crate::user::policy::BadPasswordPolicy;
use crate::user::value_object::UserState;

#[derive(Clone)]
pub struct AuthenticationDomainService<PE> {
    policy: BadPasswordPolicy,
    encoder: Arc<PE>,
}

impl<PE> AuthenticationDomainService<PE>
where
    PE: PasswordEncoder,
{
    pub fn new(policy: BadPasswordPolicy, encoder: Arc<PE>) -> Self {
        Self { policy, encoder }
    }

    pub fn policy(&self) -> &BadPasswordPolicy {
        &self.policy
    }

    pub fn authenticate(&self, user: &User, raw_password: &str) -> Result<(), DomainError> {
        match user.state() {
            UserState::Active => {}
            UserState::Locked => return Err(DomainError::UserLocked),
            UserState::Disabled => return Err(DomainError::UserDisabled),
        };

        let matches = self
            .encoder
            .verify(raw_password, user.password().as_str())?;
        if !matches {
            return Err(DomainError::BadPassword);
        }
        Ok(())
    }

    /// 依据尝试密码次数来锁定账户
    pub fn check_and_lock_user(&self, user: &mut User, security_events: &[SecurityEvent]) {
        let now = OffsetDateTime::now_utc();
        let window = Duration::from_mins(self.policy.window_minutes);
        // 找到最后一次解锁时间，作为计数起点
        let since = security_events
            .iter()
            .filter(|e| matches!(e.event_type(), SecurityEventType::UserUnlocked))
            .map(|e| e.created_at())
            .max();

        let bad_count = security_events
            .iter()
            .filter(|e| {
                e.is_bad_password_attempt()
                    && e.occurred_within(window, now)
                    && since.is_none_or(|st| e.created_at() >= st) // 只看解锁之后
            })
            .count();

        if bad_count > self.policy.max_attempts {
            user.lock();
        }
    }
}
