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

    #[error("密码错误")]
    BadPassword,

    #[error("密码格式无效")]
    InvalidPasswordFormat,

    #[error("密码编码失败：{0}")]
    PasswordEncoding(String),

    #[error("密码验证失败: {0}")]
    PasswordVerificationFailed(String),
}

#[derive(Debug, Error)]
pub enum PasswordEncoderError {
    /// 密码编码失败（如算法内部错误）
    #[error("密码编码失败: {0}")]
    EncodingFailed(String),

    /// 密码验证失败（原始密码与编码后的密码不匹配）
    #[error("密码验证失败: {0}")]
    VerificationFailed(String),

    /// 不支持的编码算法或格式
    #[error("不支持的密码编码算法或格式: {0}")]
    UnsupportedAlgorithm(String),

    /// 其他内部错误（如参数无效）
    #[error("内部错误: {0}")]
    InternalError(String),
}
