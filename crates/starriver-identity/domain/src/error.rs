use starriver_shared_base::authentication::core::error::AuthenticationError;
use thiserror::Error;

use crate::{
    password_encoder::{EncodeError, VerifyError},
    user::entity::UnAuthenticatableKind,
};

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("用户已被删除")]
    UserDeleted,

    #[error("用户已被禁用")]
    UserDisabled,

    #[error("用户已被锁定")]
    UserLocked,

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

impl From<UnAuthenticatableKind> for DomainError {
    fn from(value: UnAuthenticatableKind) -> Self {
        match value {
            UnAuthenticatableKind::Disabled => DomainError::UserDisabled,
            UnAuthenticatableKind::Deleted => DomainError::UserDeleted,
            UnAuthenticatableKind::Locked => DomainError::UserLocked,
        }
    }
}

impl From<DomainError> for AuthenticationError {
    fn from(val: DomainError) -> Self {
        match val {
            DomainError::UserDeleted => AuthenticationError::UserDeleted,
            DomainError::UserDisabled => AuthenticationError::UserDisabled,
            DomainError::UserLocked => AuthenticationError::UserLocked,
            DomainError::InvalidUsernameFormat => AuthenticationError::InvalidUsernameFormat,
            DomainError::InvalidEmailFormat => AuthenticationError::InvalidEmailFormat,
            DomainError::InvalidPasswordFormat => AuthenticationError::InvalidPasswordFormat,
            DomainError::SamePassword => AuthenticationError::MalformedRequest,
            DomainError::BadPassword => AuthenticationError::BadPassword,
            DomainError::PasswordEncoding(encode_error) => AuthenticationError::InnerError {
                message: encode_error.to_string(),
            },
            DomainError::PasswordVerification(verify_error) => AuthenticationError::InnerError {
                message: verify_error.to_string(),
            },
        }
    }
}
