use std::{env, sync::LazyLock};

use regex::Regex;
use tracing::info;

pub static REGEX_EMAIL: LazyLock<Regex> = LazyLock::new(|| {
    let regex = env::var("REGEX_EMAIL").expect("REGEX_EMAIL environment variable not set");
    info!("REGEX_EMAIl: {}", regex);
    Regex::new(regex.as_str()).unwrap_or_else(|_| panic!("{} is not a valid regex", regex))
});

pub static REGEX_USERNAME: LazyLock<Regex> = LazyLock::new(|| {
    let regex = env::var("REGEX_USERNAME").expect("REGEX_USERNAME environment variable not set");
    info!("REGEX_USERNAME: {}", regex);
    Regex::new(regex.as_str()).unwrap_or_else(|_| panic!("{} is not a valid regex", regex))
});

pub static REGEX_PASSWORD: LazyLock<Regex> = LazyLock::new(|| {
    let regex = env::var("REGEX_PASSWORD").expect("REGEX_PASSWORD environment variable not set");
    info!("REGEX_PASSWORD: {}", regex);
    Regex::new(regex.as_str()).unwrap_or_else(|_| panic!("{} is not a valid regex", regex))
});
