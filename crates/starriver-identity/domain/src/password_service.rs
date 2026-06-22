use std::sync::Arc;

use starriver_shared_base::middleware::authentication::core::error::AuthenticationError;

use crate::error::DomainError;
use crate::password_encoder::PasswordEncoder;
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

    /// # Returns
    ///
    /// - `Ok(())` - 密码匹配，用户认证成功
    /// - `Err(AuthenticationError::UserLocked)` - 用户被锁定
    /// - `Err(AuthenticationError::UserDisabled)` - 用户被禁用
    /// - `Err(AuthenticationError::BadPassword)` - 密码不匹配，且修改状态
    /// - `Err(AuthenticationError::InnerError)` - 密码编码失败
    pub fn authenticate(
        &self,
        user: &mut User,
        raw_password: &str,
    ) -> Result<(), AuthenticationError> {
        match user.state() {
            UserState::Active => {}
            UserState::Locked => return Err(AuthenticationError::UserLocked),
            UserState::Disabled => return Err(AuthenticationError::UserDisabled),
        };

        let matches = self
            .pwd_encoder
            .verify(raw_password, user.password().as_str())
            .map_err(|e| AuthenticationError::InnerError {
                message: e.to_string(),
            })?;
        if !matches {
            user.record_bad_password_and_attempt_lock(&self.pwd_policy);
            return Err(AuthenticationError::BadPassword);
        }
        Ok(())
    }

    pub fn change_password(
        &self,
        user: &mut User,
        cur_pwd: &str,
        new_pwd: &str,
    ) -> Result<(), DomainError> {
        if cur_pwd == new_pwd {
            return Err(DomainError::SamePassword);
        }
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
