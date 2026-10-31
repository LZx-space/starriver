use std::sync::Arc;

use starriver_shared_base::authentication::core::error::AuthenticationError;
use time::OffsetDateTime;

use crate::error::DomainError;
use crate::password_encoder::PasswordEncoder;
use crate::user::entity::User;
use crate::user::policy::BadPasswordPolicy;
use crate::user::specification::PasswordSpec;
use crate::user::value_object::{HashedPassword, LifeCycle};

#[derive(Clone)]
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

    /// # Returns
    ///
    /// - `Ok(())` - 密码匹配，用户认证成功
    /// - `Err(AuthenticationError::UserLocked)` - 用户被锁定
    /// - `Err(AuthenticationError::UserDisabled)` - 用户被禁用
    /// - `Err(AuthenticationError::UserDeleted)` - 用户已删除
    /// - `Err(AuthenticationError::BadPassword)` - 密码不匹配，并记录失败次数（达上限锁定）
    /// - `Err(AuthenticationError::InnerError)` - 密码校验失败（编码器故障）
    pub fn authenticate(
        &self,
        user: &mut User,
        raw_password: &str,
    ) -> Result<(), AuthenticationError> {
        match user.life_cycle() {
            LifeCycle::Active => {}
            LifeCycle::Disabled => return Err(AuthenticationError::UserDisabled),
            LifeCycle::Deleted => return Err(AuthenticationError::UserDeleted),
        };

        if user.is_locked() {
            return Err(AuthenticationError::UserLocked);
        }

        let matches = self
            .pwd_encoder
            .verify(raw_password, user.password().as_str())
            .map_err(|e| AuthenticationError::InnerError {
                message: e.to_string(),
            })?;
        if !matches {
            user.record_password_attempts(&self.pwd_policy, OffsetDateTime::now_utc());
            return Err(AuthenticationError::BadPassword);
        }
        Ok(())
    }

    /// # params
    /// - `cur_pwd` is the current password of the user
    /// - `new_pwd` is the new password that the user wants to use
    ///
    /// # returns
    /// - `Ok(())` - password changed successfully
    /// - `Err(DomainError::UserDisabled)` - user is disabled or deleted
    /// - `Err(DomainError::UserLocked)` - user is locked
    /// - `Err(DomainError::SamePassword)` - new password is same as current password
    /// - `Err(DomainError::BadPassword)` - incorrect current password
    /// - `Err(DomainError::InvalidPasswordFormat)` - invalid password format
    /// - `Err(DomainError::PasswordEncoding(_))` - password verification or encoding failed
    /// - `Err(DomainError::InvalidPasswordHash)` - encoder produced an invalid hash
    ///
    pub fn change_password(
        &self,
        user: &mut User,
        cur_pwd: &str,
        new_pwd: &str,
    ) -> Result<(), DomainError> {
        match user.life_cycle() {
            LifeCycle::Active => {}
            LifeCycle::Disabled | LifeCycle::Deleted => return Err(DomainError::UserDisabled),
        }
        if user.is_locked() {
            return Err(DomainError::UserLocked);
        }
        if cur_pwd == new_pwd {
            return Err(DomainError::SamePassword);
        }
        let matches = self.pwd_encoder.verify(cur_pwd, user.password().as_str())?;
        if !matches {
            user.record_password_attempts(&self.pwd_policy, OffsetDateTime::now_utc());
            return Err(DomainError::BadPassword);
        }
        self.pwd_spec.validate(new_pwd)?;
        let hashed_pwd = self.pwd_encoder.encode(new_pwd)?;
        let password = HashedPassword::new(hashed_pwd)?;

        user.change_password(password);
        user.reset_password_attempts();
        Ok(())
    }

    pub fn reset_password(&self, user: &mut User, new_pwd: &str) -> Result<(), DomainError> {
        self.pwd_spec.validate(new_pwd)?;
        let hashed_pwd = self.pwd_encoder.encode(new_pwd)?;
        let password = HashedPassword::new(hashed_pwd)?;

        user.change_password(password);
        Ok(())
    }
}
