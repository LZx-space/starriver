use serde::Deserialize;

#[derive(Deserialize)]
pub struct BloggingConfig {
    pub cache: Cache,
}

#[derive(Deserialize)]
pub struct Cache {
    pub base_cache_ttl_sec: u64,
    /// from 0 to this value will be randomly added to the cache TTL
    pub base_cache_jitter_sec_range: u64,

    #[serde(default = "default_post_page_cache_max_capacity")]
    pub post_page_cache_max_capacity: u64,
}

fn default_post_page_cache_max_capacity() -> u64 {
    128
}
