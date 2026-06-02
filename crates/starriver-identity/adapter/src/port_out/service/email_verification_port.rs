use std::time::Duration;

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::Mailbox,
    transport::smtp::authentication::Credentials,
};
use moka::future::Cache;
use starriver_identity_application::{
    error::EmailVerificationError, port_out::email_verification_port::EmailVerificationPort,
};

use crate::config::SmtpVerification;

pub struct SmtpVerificationPort {
    smtp_client: AsyncSmtpTransport<Tokio1Executor>,
    smtp_username: String,
    code_cache: Cache<String, String>,
}

impl SmtpVerificationPort {
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

        Ok(Self {
            smtp_client,
            smtp_username: cfg.smtp_username.clone(),
            code_cache,
        })
    }
}

impl EmailVerificationPort for SmtpVerificationPort {
    async fn send_code(&self, email: &str) -> Result<(), EmailVerificationError> {
        let to = email
            .parse::<Mailbox>()
            .map_err(|e| EmailVerificationError::SendCodeError(e.to_string()))?;
        let from = self
            .smtp_username
            .parse::<Mailbox>()
            .map_err(|e| EmailVerificationError::SendCodeError(e.to_string()))?;

        let code: String = (0..6)
            .map(|_| rand::random::<u8>() % 10 + b'0')
            .map(|b| b as char)
            .collect();

        let message = Message::builder()
            .subject("Starriver User's Email Verification")
            .from(from)
            .to(to)
            .body(format!("email verification code is {}", code))
            .map_err(|e| EmailVerificationError::SendCodeError(e.to_string()))?;
        self.smtp_client
            .send(message)
            .await
            .map(|_| ())
            .map_err(|e| EmailVerificationError::SendCodeError(e.to_string()))?;

        self.code_cache
            .insert(email.to_string(), code.clone())
            .await;
        Ok(())
    }

    async fn validate_code(&self, email: &str, code: &str) -> Result<bool, EmailVerificationError> {
        match self.code_cache.get(email).await {
            Some(cached_code) => Ok(cached_code == code),
            None => Ok(false),
        }
    }
}
