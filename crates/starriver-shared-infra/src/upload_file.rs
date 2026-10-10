use std::path::Path;
use std::sync::Arc;

use starriver_shared_base::upload_file::UploadLocationResolver;

use crate::config::Uploads;

#[derive(Clone)]
pub struct DefaultUploadLocationResolver {
    uploads: Arc<Uploads>,
}

impl DefaultUploadLocationResolver {
    pub fn new(uploads: Arc<Uploads>) -> Self {
        Self { uploads }
    }

    /// 只允许普通文件名：剥离任何路径部分（`../x`、`a/b`、`a\b`），
    /// 防止 file_name 携带路径分隔符或 `..` 越出存储目录
    fn safe_file_name<'a>(&self, file_name: &'a str) -> &'a str {
        Path::new(file_name)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
    }
}

impl UploadLocationResolver for DefaultUploadLocationResolver {
    fn url(&self, file_name: &str) -> String {
        let safe_name = self.safe_file_name(file_name);
        format!("{}/{}", self.uploads.proxy_prefix, safe_name)
    }

    fn save_path(&self, file_name: &str) -> std::path::PathBuf {
        Path::new(&self.uploads.storage_dir).join(self.safe_file_name(file_name))
    }
}
