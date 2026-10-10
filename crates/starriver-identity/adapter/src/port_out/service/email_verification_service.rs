use std::time::Duration;

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::Mailbox,
    transport::smtp::authentication::Credentials,
};
use moka::future::Cache;
use rand::random_range;
use starriver_identity_application::{
    error::EmailVerificationError, port::email_verification_service::EmailVerificationService,
};
use starriver_identity_domain::user::value_object::Email;

use crate::config::SmtpVerification;

pub struct SmtpVerificationService {
    smtp_client: AsyncSmtpTransport<Tokio1Executor>,
    smtp_username: String,
    code_cache: Cache<String, String>,
    send_cooldown_cache: Cache<String, ()>,
    validate_attempts_cache: Cache<String, u8>,
}

impl SmtpVerificationService {
    const MAX_VALIDATE_ATTEMPTS: u8 = 5;

    pub fn new(cfg: &SmtpVerification) -> Result<Self, EmailVerificationError> {
        let creds = Credentials::new(cfg.smtp_username.clone(), cfg.smtp_password.clone());
        let smtp_client = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&cfg.smtp_host)
            .map_err(|e| EmailVerificationError::BuildClientError(e.to_string()))?
            .port(cfg.smtp_port)
            .credentials(creds)
            .build();

        let code_cache = Cache::builder()
            .max_capacity(cfg.code_cache_max_capacity)
            .time_to_live(Duration::from_hours(cfg.code_cache_ttl_hours))
            .build();
        let send_cooldown_cache = Cache::builder()
            .max_capacity(cfg.code_cache_max_capacity)
            .time_to_live(Duration::from_secs(cfg.code_send_cooldown_secs))
            .build();
        let validate_attempts_cache = Cache::builder()
            .max_capacity(cfg.code_cache_max_capacity)
            .time_to_live(Duration::from_hours(cfg.code_cache_ttl_hours))
            .build();

        Ok(Self {
            smtp_client,
            smtp_username: cfg.smtp_username.clone(),
            code_cache,
            send_cooldown_cache,
            validate_attempts_cache,
        })
    }
}

impl EmailVerificationService for SmtpVerificationService {
    async fn send_code(&self, email_to: &str) -> Result<(), EmailVerificationError> {
        let email_to = Email::normalize(email_to);

        if self.send_cooldown_cache.contains_key(&email_to) {
            return Err(EmailVerificationError::SendCodeError(
                "Too many requests. Please try again later.".to_string(),
            ));
        }

        let from = self
            .smtp_username
            .parse::<Mailbox>()
            .map_err(|e| EmailVerificationError::SendCodeError(e.to_string()))?;

        let to = email_to
            .parse::<Mailbox>()
            .map_err(|e| EmailVerificationError::SendCodeError(e.to_string()))?;

        let code: String = (0..6).map(|_| random_range(b'0'..=b'9') as char).collect();

        let message = Message::builder()
            .subject("Starriver User's Email Verification")
            .from(from)
            .to(to)
            .body(format!("Email verification code is {}", code))
            .map_err(|e| EmailVerificationError::SendCodeError(e.to_string()))?;

        self.smtp_client
            .send(message)
            .await
            .map(|_| ())
            .map_err(|e| EmailVerificationError::SendCodeError(e.to_string()))?;

        // 新码开启新的尝试窗口：重置失败计数，否则旧计数会立即作废新码
        self.validate_attempts_cache.invalidate(&email_to).await;
        self.code_cache.insert(email_to.clone(), code).await;
        self.send_cooldown_cache.insert(email_to, ()).await;
        Ok(())
    }

    async fn validate_code(&self, email: &str, code: &str) -> Result<bool, EmailVerificationError> {
        let email = Email::normalize(email);

        let Some(cached_code) = self.code_cache.get(&email).await else {
            return Ok(false);
        };

        if cached_code == code {
            // 成功即消费：验证码与尝试计数一并清掉
            self.code_cache.invalidate(&email).await;
            self.validate_attempts_cache.invalidate(&email).await;
            return Ok(true);
        }

        let attempts = self
            .validate_attempts_cache
            .get_with_by_ref(&email, async { 0 })
            .await
            + 1;
        if attempts >= Self::MAX_VALIDATE_ATTEMPTS {
            // 达到上限：作废验证码并清空计数，防止继续枚举
            self.code_cache.invalidate(&email).await;
            self.validate_attempts_cache.invalidate(&email).await;
        } else {
            self.validate_attempts_cache.insert(email, attempts).await;
        }
        Ok(false)
    }
}
