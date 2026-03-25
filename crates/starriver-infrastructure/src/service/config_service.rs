use std::path::PathBuf;

use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let project_config = find_config("settings.toml")
        .ok_or(ConfigError::Message("config file not found".to_string()))?;
    let builder = Config::builder().add_source(
        File::from(project_config)
            .format(FileFormat::Toml)
            .required(true),
    );
    builder.build()?.try_deserialize()
}

fn find_config(filename: &str) -> Option<PathBuf> {
    let mut current_dir = std::env::current_dir().ok()?;
    loop {
        let target = current_dir.join(filename);
        if target.exists() {
            return Some(target);
        }
        if !current_dir.pop() {
            break;
        }
    }
    None
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub http_server: HttpServer,
    pub database: Database,
    pub email: Email,
    pub regex: Regex,
    pub email_verification_cache: EmailVerificationCache,
}

#[derive(Debug, Deserialize)]
pub struct HttpServer {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Email {
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Regex {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct EmailVerificationCache {
    pub max_capacity: u64,
    pub ttl_hours: u64,
}

///////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load_config();
        assert!(config.is_ok());
        let config = config.unwrap();
        println!("{:?}", config);
    }
}
