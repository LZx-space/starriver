use thiserror::Error;

use crate::password_encoder::{EncodeError, VerifyError};

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("用户已被锁定")]
    UserLocked,

    #[error("用户已被禁用")]
    UserDisabled,

    #[error("用户名格式无效")]
    InvalidUsernameFormat,

    #[error("邮箱格式无效")]
    InvalidEmailFormat,

    #[error("密码格式无效")]
    InvalidPasswordFormat,

    #[error("当前密码与新密码相同")]
    SamePassword,

    #[error("密码错误")]
    BadPassword,

    #[error(transparent)]
    PasswordEncoding(#[from] EncodeError),

    #[error(transparent)]
    PasswordVerification(#[from] VerifyError),
}
