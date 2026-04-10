use std::path::{MAIN_SEPARATOR, PathBuf};

use config::{Config, ConfigError, File, FileFormat};
use serde::{Deserialize, Deserializer};

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
    pub assets: Assets,
    pub email: Email,
    pub regex: Regex,
    pub aggregate: Aggregate,
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
    pub smtp: EmailSmtp,
    pub verification_cache: EmailVerificationCache,
}

#[derive(Debug, Deserialize)]
pub struct EmailSmtp {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct EmailVerificationCache {
    pub max_capacity: u64,
    pub ttl_hours: u64,
}

#[derive(Debug, Deserialize)]
pub struct Regex {
    pub email: String,
    pub username: String,
    pub password: String,
}

///////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct Aggregate {
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub policy: UserPolicy,
}

#[derive(Debug, Deserialize)]
pub struct UserPolicy {
    pub bad_password_window_mins: u64,
    pub max_bad_password_attempts: usize,
}

///////////////////////////////////////////////////////////////////////////////////

/// 静态资源
#[derive(Clone, Debug, Deserialize)]
pub struct Assets {
    #[serde(deserialize_with = "validate_path_separators")]
    pub static_base_dir: String,
    /// 上传专用目录
    pub uploads: Uploads,
}

/// 上传文件配置
#[derive(Clone, Debug, Deserialize)]
pub struct Uploads {
    #[serde(deserialize_with = "validate_path_separators")]
    pub relative_dir: String,
}

/// 验证字符串不以路径分隔符开头或结尾
fn validate_path_separators<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let separators = ['/', '\\', MAIN_SEPARATOR]; // 覆盖 Unix 和 Windows
    if s.starts_with(separators) {
        return Err(serde::de::Error::custom(format!(
            "dir must not start with a path separator, got: {}",
            s
        )));
    }
    if s.ends_with(separators) {
        return Err(serde::de::Error::custom(format!(
            "dir must not end with a path separator, got: {}",
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
        let config = config.unwrap();
        println!("{:?}", config);
    }
}
