use std::env::VarError;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use starriver_blogging_adapter::config::BloggingConfig;
use starriver_identity_adapter::config::IdentityConfig;
use starriver_shared_infra::config::{Auth, Uploads};

const APP_CONFIG_PATH_ENV: &str = "APP_CONFIG_PATH";

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let config_file_path = match std::env::var(APP_CONFIG_PATH_ENV) {
        Ok(path) => path,
        Err(VarError::NotPresent) => {
            println!("cfg file env var {APP_CONFIG_PATH_ENV} not set, try read config-dev file");
            "config-dev".to_owned()
        }
        Err(VarError::NotUnicode(value)) => Err(ConfigError::Message(format!(
            "{APP_CONFIG_PATH_ENV} 不是合法 Unicode 路径: {value:?}"
        )))?,
    };
    Config::builder()
        .add_source(File::with_name(&config_file_path).required(true)) // 外部路径
        .add_source(Environment::with_prefix("APP").separator("__")) // 环境变量最高优先级
        .build()?
        .try_deserialize()
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub http_server: HttpServer,
    pub database: Database,
    pub logging: Logging,
    pub csrf: Csrf,
    pub auth: Auth,
    pub uploads: Uploads,
    pub ctx_identity: IdentityConfig,
    pub ctx_blogging: BloggingConfig,
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
pub struct Logging {
    /// true-file，false-console
    pub file_enabled: bool,
    pub file_directory: String,
    pub file_name_prefix: String,
    pub max_files: usize,
}

#[derive(Deserialize)]
pub struct Csrf {
    pub enabled: bool,
    pub trusted_origins: Vec<String>,
}
