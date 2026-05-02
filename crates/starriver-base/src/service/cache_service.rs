use std::{sync::Arc, time::Duration};

use moka::future::Cache;

use crate::{
    error::{ApiError, Cause},
    service::config_service::EmailVerificationCache,
};

#[derive(Clone)]
pub struct VerificationCodeCache {
    inner: Arc<Cache<String, String>>,
}

impl VerificationCodeCache {
    pub fn new(cfg: EmailVerificationCache) -> Self {
        Self {
            inner: Cache::builder()
                .max_capacity(cfg.max_capacity)
                .time_to_live(Duration::from_hours(cfg.ttl_hours))
                .build()
                .into(),
        }
    }

    pub async fn cache_email_verification_code(&self, email: &str) -> String {
        let code: String = (0..6)
            .map(|_| rand::random::<u8>() % 10 + b'0')
            .map(|b| b as char)
            .collect();
        self.inner.insert(email.to_string(), code.clone()).await;
        code
    }

    pub async fn verify_email_by_verification_code(
        &self,
        email: &str,
        code: &str,
    ) -> Result<(), ApiError> {
        if let Some(cached_code) = self.inner.get(email).await
            && cached_code == code
        {
            return Ok(());
        }
        Err(ApiError::new(
            Cause::ClientBadRequest,
            "verification code does not match or invalid",
        ))
    }
}

///////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_verification_code() {
        let cache = VerificationCodeCache::new(EmailVerificationCache {
            max_capacity: 100,
            ttl_hours: 60,
        });
        let email = "test@example.com";
        let code = cache.cache_email_verification_code(email).await;
        assert_eq!(code.len(), 6);
        let result = cache
            .verify_email_by_verification_code(email, code.as_str())
            .await;
        assert!(result.is_ok(), "verification code should match")
    }
}
