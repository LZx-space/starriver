use std::sync::Arc;

use starriver_shared_base::authentication::core::error::AuthenticationError;
use time::OffsetDateTime;

use crate::error::DomainError;
use crate::password_encoder::PasswordEncoder;
use crate::user::entity::User;
use crate::user::policy::BadPasswordPolicy;
use crate::user::value_object::{AccountStatus, HashedPassword, RawPassword};

#[derive(Clone)]
pub struct PasswordDomainService<PE> {
    pwd_policy: BadPasswordPolicy,
    pwd_encoder: Arc<PE>,
}

impl<PE> PasswordDomainService<PE>
where
    PE: PasswordEncoder,
{
    pub fn new(pwd_policy: BadPasswordPolicy, pwd_encoder: Arc<PE>) -> Self {
        Self {
            pwd_policy,
            pwd_encoder,
        }
    }

    /// # Parameters
    ///
    /// - `user`: 用户
    /// - `raw_password`: 原始密码，不用值类型，避免动态修改规则老密码无法传入
    ///
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
        match user.status() {
            AccountStatus::Active => {}
            AccountStatus::Disabled => return Err(AuthenticationError::UserDisabled),
            AccountStatus::Deleted => return Err(AuthenticationError::UserDeleted),
        };

        if user.is_locked() {
            return Err(AuthenticationError::UserLocked);
        }

        let matches = user
            .password()
            .verify(raw_password, self.pwd_encoder.as_ref())
            .map_err(|e| AuthenticationError::InnerError {
                message: e.to_string(),
            })?;
        if !matches {
            user.record_password_attempts(&self.pwd_policy, OffsetDateTime::now_utc());
            return Err(AuthenticationError::BadPassword);
        }
        user.reset_password_attempts();
        Ok(())
    }

    /// # params
    /// - `cur_pwd` the current password of the user
    /// - `new_pwd` the new password that the user wants to use
    ///
    /// # returns
    /// - `Ok(())` - password changed successfully
    /// - `Err(DomainError::UserDisabled)` - user is disabled or deleted
    /// - `Err(DomainError::UserLocked)` - user is locked
    /// - `Err(DomainError::SamePassword)` - new password is same as current password
    /// - `Err(DomainError::BadPassword)` - incorrect current password
    /// - `Err(DomainError::InvalidPasswordFormat)` - invalid password format
    pub fn change_password(
        &self,
        user: &mut User,
        cur_pwd: &str,
        new_pwd: &RawPassword,
    ) -> Result<(), DomainError> {
        match user.status() {
            AccountStatus::Active => {}
            AccountStatus::Disabled | AccountStatus::Deleted => {
                return Err(DomainError::UserDisabled);
            }
        }
        if user.is_locked() {
            return Err(DomainError::UserLocked);
        }
        if cur_pwd == new_pwd.as_str() {
            return Err(DomainError::SamePassword);
        };
        let matches = user
            .password()
            .verify(cur_pwd, self.pwd_encoder.as_ref())
            .is_ok();
        if !matches {
            user.record_password_attempts(&self.pwd_policy, OffsetDateTime::now_utc());
            return Err(DomainError::BadPassword);
        }
        let password = HashedPassword::from_raw(new_pwd, self.pwd_encoder.as_ref())?;

        user.change_password(password);
        user.reset_password_attempts();
        Ok(())
    }

    pub fn reset_password(
        &self,
        user: &mut User,
        new_pwd: &RawPassword,
    ) -> Result<(), DomainError> {
        let password = HashedPassword::from_raw(new_pwd, self.pwd_encoder.as_ref())?;

        user.change_password(password);
        user.reset_password_attempts();
        Ok(())
    }
}
