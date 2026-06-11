use std::{path::MAIN_SEPARATOR, sync::Arc};

use serde::{Deserialize, Deserializer};

#[derive(Clone, Deserialize)]
pub struct Auth {
    pub jws_exp_hours: u16,
    pub jws_cookie_name: String,
    pub jws_secret: Arc<String>,
}

impl Auth {
    pub fn jws_secret_as_ref(&self) -> &[u8] {
        self.jws_secret.as_bytes()
    }
}

/// 上传文件配置
#[derive(Clone, Deserialize)]
pub struct Uploads {
    /// 上传文件在磁盘上的存储根目录（不以分隔符结束，相对或绝对路径）
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
