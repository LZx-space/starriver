use starriver_shared_base::upload_file::UploadLocationResolver;

use crate::config::Uploads;

pub struct DefaultUploadLocationResolver {
    uploads: Uploads,
}

impl DefaultUploadLocationResolver {
    pub fn new(uploads: Uploads) -> Self {
        Self { uploads }
    }
}

impl UploadLocationResolver for DefaultUploadLocationResolver {
    fn url(&self, file_name: &str) -> String {
        format!("{}/{}", self.uploads.proxy_prefix, file_name)
    }

    fn save_path(&self, file_name: &str) -> std::path::PathBuf {
        format!("{}/{}", self.uploads.storage_dir, file_name).into()
    }
}
