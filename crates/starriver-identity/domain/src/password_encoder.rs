use thiserror::Error;

pub trait PasswordEncoder {
    fn encode(&self, password: &str) -> Result<String, EncodeError>;

    /// Ok(true)  = 密码匹配
    /// Ok(false) = 密码不匹配（用户输错了）
    /// Err(...)  = 基础设施故障
    fn verify(&self, raw_password: &str, encoded_password: &str) -> Result<bool, VerifyError>;
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("密码哈希失败")]
    Hashing(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("密码校验失败")]
    Verification(#[source] Box<dyn std::error::Error + Send + Sync>),
}
