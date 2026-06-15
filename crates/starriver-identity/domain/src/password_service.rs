use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;

use crate::error::DomainError;
use crate::password_encoder::PasswordEncoder;
use crate::security_event::entity::SecurityEvent;
use crate::security_event::value_object::SecurityEventType;
use crate::user::entity::User;
use crate::user::policy::BadPasswordPolicy;
use crate::user::specification::PasswordSpec;
use crate::user::value_object::{HashedPassword, UserState};

pub struct PasswordDomainService<PE> {
    pwd_policy: BadPasswordPolicy,
    pwd_encoder: Arc<PE>,
    pwd_spec: Arc<PasswordSpec>,
}

impl<PE> PasswordDomainService<PE>
where
    PE: PasswordEncoder,
{
    pub fn new(
        pwd_policy: BadPasswordPolicy,
        pwd_encoder: Arc<PE>,
        pwd_spec: Arc<PasswordSpec>,
    ) -> Self {
        Self {
            pwd_policy,
            pwd_encoder,
            pwd_spec,
        }
    }

    pub fn policy(&self) -> &BadPasswordPolicy {
        &self.pwd_policy
    }

    pub fn authenticate(&self, user: &User, raw_password: &str) -> Result<(), DomainError> {
        match user.state() {
            UserState::Active => {}
            UserState::Locked => return Err(DomainError::UserLocked),
            UserState::Disabled => return Err(DomainError::UserDisabled),
        };

        let matches = self
            .pwd_encoder
            .verify(raw_password, user.password().as_str())?;
        if !matches {
            return Err(DomainError::BadPassword);
        }
        Ok(())
    }

    /// 依据尝试密码次数来锁定账户
    pub fn check_and_lock_user(&self, user: &mut User, security_events: &[SecurityEvent]) {
        let now = OffsetDateTime::now_utc();
        let window = Duration::from_mins(self.pwd_policy.window_minutes);
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

        if bad_count > self.pwd_policy.max_attempts {
            user.lock();
        }
    }

    pub fn change_password(
        &self,
        user: &mut User,
        cur_pwd: &str,
        new_pwd: &str,
    ) -> Result<(), DomainError> {
        let matches = self.pwd_encoder.verify(cur_pwd, user.password().as_str())?;
        if !matches {
            return Err(DomainError::BadPassword);
        }
        self.pwd_spec.validate(new_pwd)?;
        let hashed_pwd = &self.pwd_encoder.encode(new_pwd)?;
        let password = HashedPassword::new(hashed_pwd)?;

        user.change_password(password);
        Ok(())
    }
}
