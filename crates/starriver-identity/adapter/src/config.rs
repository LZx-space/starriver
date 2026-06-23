use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdentityConfig {
    pub regexes: Regexes,
    pub bad_password: BadPassword,
    pub email_smtp: SmtpVerification,
}

#[derive(Deserialize)]
pub struct Regexes {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct BadPassword {
    pub window_minutes: u16,
    pub max_attempts: u8,
    pub lockout_minutes: u16,
}

#[derive(Deserialize)]
pub struct SmtpVerification {
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub code_cache_max_capacity: u64,
    pub code_cache_ttl_hours: u64,
}
