use std::{env, sync::OnceLock};

use regex::Regex;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

// todo 懒初始化会panic的变量有风险，启动时做完验证
pub fn email_regex() -> &'static Regex {
    if let Some(conn) = EMAIL_REGEX.get() {
        return conn;
    }
    let email_regex = env::var("REGEX_EMAIL").expect("REGEX_EMAIL environment variable not set");
    let regex = Regex::new(email_regex.as_str())
        .unwrap_or_else(|_| panic!("{} is not a valid regex", email_regex));
    EMAIL_REGEX.get_or_init(|| regex)
}
