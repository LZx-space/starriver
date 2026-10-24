use crate::error::EmailVerificationError;

pub trait EmailVerificationService {
    /// 生成验证码、存储（含过期时间）并通过邮件发送给指定地址
    fn send_code(
        &self,
        email_to: &str,
    ) -> impl Future<Output = Result<(), EmailVerificationError>> + Send;

    /// 校验验证码是否有效（匹配、未过期、未超尝试次数），**不消费**。
    /// 错误尝试的计数在这里累加；达到上限时作废验证码。
    fn check_code(
        &self,
        email_to: &str,
        code: &str,
    ) -> impl Future<Output = Result<bool, EmailVerificationError>> + Send;

    /// 业务成功后消费验证码。只删除仍然匹配的那一个码，
    /// 避免误删校验与消费之间新发的验证码。
    fn consume_code(
        &self,
        email_to: &str,
        code: &str,
    ) -> impl Future<Output = Result<(), EmailVerificationError>> + Send;
}
