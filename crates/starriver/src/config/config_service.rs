use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use starriver_identity_adapter::config::IdentityConfig;

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let config_path = std::env::var("APP_CONFIG_PATH").unwrap_or_else(|_| "config-dev.toml".into());

    Config::builder()
        .add_source(File::with_name("config-dev").required(false)) // 默认配置
        .add_source(File::with_name(&config_path).required(false)) // 外部路径
        .add_source(Environment::with_prefix("APP").separator("__")) // 环境变量最高优先级
        .build()?
        .try_deserialize()
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub http_server: HttpServer,
    pub database: Database,
    pub ctx_identity_cfg: IdentityConfig,
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
