use std::path::PathBuf;

use uuid::Uuid;

/// Resolves the location of an uploaded file.
pub trait UploadLocationResolver: Send + Sync {
    /// Returns the URL of the uploaded file.
    fn url(&self, file_name: &str) -> String;
    /// Returns the save path of the file_name.
    fn file_path(&self, file_name: &str) -> PathBuf;
    /// Returns the temp save path of the uploaded file.
    fn temp_file_path(&self, upload_id: &Uuid) -> PathBuf;
}
