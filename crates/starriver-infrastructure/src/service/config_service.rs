use std::path::{MAIN_SEPARATOR, PathBuf};

use config::{Config, ConfigError, File, FileFormat};
use serde::{Deserialize, Deserializer};

use crate::security::authentication::web::config::AuthConfig;

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let project_config = find_config("config.toml")
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

#[derive(Deserialize)]
pub struct AppConfig {
    pub http_server: HttpServer,
    pub database: Database,
    pub uploads: Uploads,
    pub auth: AuthConfig,
    pub email: Email,
    pub regex: Regex,
    pub aggregate: Aggregate,
    pub file_logging: FileLogging,
}

#[derive(Deserialize)]
pub struct HttpServer {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Deserialize)]
pub struct Email {
    pub smtp: EmailSmtp,
    pub verification_cache: EmailVerificationCache,
}

#[derive(Deserialize)]
pub struct EmailSmtp {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct EmailVerificationCache {
    pub max_capacity: u64,
    pub ttl_hours: u64,
}

#[derive(Deserialize)]
pub struct Regex {
    pub email: String,
    pub username: String,
    pub password: String,
}

///////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
pub struct FileLogging {
    pub file_enabled: bool,
    pub file_directory: String,
    pub file_name_prefix: String,
    pub max_files: usize,
}

///////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
pub struct Aggregate {
    pub user: User,
}

#[derive(Deserialize)]
pub struct User {
    pub policy: UserPolicy,
}

#[derive(Deserialize)]
pub struct UserPolicy {
    pub bad_password_window_mins: u64,
    pub max_bad_password_attempts: usize,
}

///////////////////////////////////////////////////////////////////////////////////

/// 上传文件配置
#[derive(Clone, Deserialize)]
pub struct Uploads {
    /// 上传文件在磁盘上的存储根目录（绝对路径或相对工程根目录的路径）
    #[serde(deserialize_with = "not_end_with_separator")]
    pub storage_dir: String,
    /// Nginx 或其他前端服务访问上传文件时使用的 URL 路径前缀（例如 "/uploads"）
    #[serde(deserialize_with = "start_and_not_end_with_separator")]
    pub proxy_prefix: String,
}

fn not_end_with_separator<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let separators = ['/', '\\', MAIN_SEPARATOR]; // 覆盖 Unix 和 Windows
    if s.ends_with(separators) {
        return Err(serde::de::Error::custom(format!(
            "must not end with a path separator, got: {}",
            s
        )));
    }
    Ok(s)
}

fn start_and_not_end_with_separator<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let separators = ['/', '\\', MAIN_SEPARATOR]; // 覆盖 Unix 和 Windows
    if !s.starts_with(separators) {
        return Err(serde::de::Error::custom(format!(
            "must start with a path separator, got: {}",
            s
        )));
    }
    if s.ends_with(separators) {
        return Err(serde::de::Error::custom(format!(
            "must not end with a path separator, got: {}",
            s
        )));
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load_config();
        assert!(config.is_ok());
    }
}
