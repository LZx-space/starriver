use thiserror::Error;

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

    #[error("密码哈希无效")]
    InvalidPasswordHash,

    #[error("当前密码与新密码相同")]
    SamePassword,

    #[error("密码错误")]
    BadPassword,

    #[error(transparent)]
    PasswordEncoding(#[from] PasswordEncoderError),
}

#[derive(Debug, Error)]
pub enum PasswordEncoderError {
    #[error("密码哈希失败：{0}")]
    EncodingFailed(String),

    #[error("密码校验失败：{0}")]
    VerificationFailed(String),
}
