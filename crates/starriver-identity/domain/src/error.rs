use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("用户被锁定")]
    UserLocked,

    #[error("用户被禁用")]
    UserDisabled,

    #[error("用户名格式无效")]
    InvalidUsernameFormat,

    #[error("邮箱格式无效")]
    InvalidEmailFormat,

    #[error("密码格式无效")]
    InvalidPasswordFormat,

    #[error("密码错误")]
    BadPassword,

    #[error(transparent)]
    PasswordEncoding(#[from] PasswordEncoderError),
}

#[derive(Debug, Error)]
pub enum PasswordEncoderError {
    #[error("密码编码失败: {0}")]
    EncodingFailed(String),

    #[error("校验密码失败: {0}")]
    VerificationFailed(String),
}
