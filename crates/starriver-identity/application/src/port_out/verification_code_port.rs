use crate::common::error::{SendVerificationCodeError, ValidateVerificationCodeError};

pub trait VerificationCodePort {
    /// 生成验证码、存储（含过期时间）并通过邮件发送给指定地址
    fn send_code(
        &self,
        email: &str,
    ) -> impl Future<Output = Result<(), SendVerificationCodeError>> + Send;

    /// 验证提交的验证码是否有效（匹配、未过期、未超尝试次数）
    fn validate_code(
        &self,
        email: &str,
        code: &str,
    ) -> impl Future<Output = Result<(), ValidateVerificationCodeError>> + Send;
}
