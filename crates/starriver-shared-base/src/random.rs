use rand::Rng;
use std::{ops::Add, range::Range, time::Duration};

/// Returns a random value within the given range using the thread-local RNG.
/// Includes the range endpoints.
pub fn jitter(range: &Range<u64>) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range.start..=range.end)
}

pub fn duration_add_jitter(duration: &Duration, jitter_sec_range: &Range<u64>) -> Duration {
    let jitter = jitter(jitter_sec_range);
    let sec = duration.as_secs().add(jitter);
    Duration::from_secs(sec)
}

pub fn duration_with_jitter(base_sec: u64, jitter_sec_range: u64) -> Duration {
    let cache_ttl = Duration::from_secs(base_sec);
    let jitter_range = Range {
        start: 0,
        end: jitter_sec_range,
    };
    duration_add_jitter(&cache_ttl, &jitter_range)
}
