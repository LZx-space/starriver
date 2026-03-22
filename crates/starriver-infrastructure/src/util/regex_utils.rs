use std::{env, sync::OnceLock};

use regex::Regex;
use tracing::info;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn email_regex() -> &'static Regex {
    if let Some(conn) = EMAIL_REGEX.get() {
        return conn;
    }
    let email_regex = env::var("REGEX_EMAIL").expect("REGEX_EMAIL environment variable not set");
    info!("REGEX_EMAIL: {}", email_regex);
    let regex = Regex::new(email_regex.as_str()).expect("REGEX_EMAIL is not a valid regex");
    EMAIL_REGEX.get_or_init(|| regex)
}
