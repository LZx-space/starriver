use std::path::PathBuf;

/// Resolves the location of an uploaded file.
pub trait UploadLocationResolver: Send + Sync {
    /// Returns the URL of the uploaded file.
    fn url(&self, file_name: &str) -> String;
    /// Returns the save path of the uploaded file.
    fn save_path(&self, file_name: &str) -> PathBuf;
}
