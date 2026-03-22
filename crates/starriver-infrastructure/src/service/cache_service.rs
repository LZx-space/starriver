use moka::future::Cache;
use std::{env, sync::OnceLock, time::Duration};
use uuid::Uuid;

use crate::error::{ApiError, Cause};

static VERIFICATION_CODE_CACHE: OnceLock<Cache<Uuid, String>> = OnceLock::new();

// keep async for future expansion
pub async fn get_or_init_verification_code_cache() -> &'static Cache<Uuid, String> {
    VERIFICATION_CODE_CACHE.get_or_init(|| {
        let max_capacity = env::var("DB_URL").unwrap_or("10_000".to_string());
        let max_capacity = max_capacity.parse::<u64>().unwrap_or(10_000);
        Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_mins(30))
            .build()
    })
}

pub async fn cache_email_verification_code(u_id: Uuid) {
    let code: String = (0..6)
        .map(|_| rand::random::<u8>() % 10 + b'0')
        .map(|b| b as char)
        .collect();
    get_or_init_verification_code_cache()
        .await
        .insert(u_id, code)
        .await;
}

pub async fn verify_email_verification_code(u_id: Uuid, code: String) -> Result<(), ApiError> {
    let cache = get_or_init_verification_code_cache().await;
    if let Some(cached_code) = cache.get(&u_id).await
        && cached_code == code
    {
        return Ok(());
    }
    Err(ApiError::new(
        Cause::ClientBadRequest,
        "Verification code does not match or invalid",
    ))
}
