use std::path::Path;

/// # return
/// * 文件格式或者空
pub fn get_extension(filename: &str) -> &str {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
}
