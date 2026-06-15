use crate::error::PasswordEncoderError;

pub trait PasswordEncoder {
    fn encode(&self, password: &str) -> Result<String, PasswordEncoderError>;

    /// Ok(true)  = 密码匹配
    /// Ok(false) = 密码不匹配（用户输错了）
    /// Err(...)  = 基础设施故障
    fn verify(
        &self,
        raw_password: &str,
        encoded_password: &str,
    ) -> Result<bool, PasswordEncoderError>;
}
