use std::path::Path;

use bytes::Bytes;
use tracing::info;

use crate::error::ApiError;

/// 将数据写入文件
pub async fn write_to_file(target_dir: &Path, filename: &str, data: Bytes) -> Result<(), ApiError> {
    // 保存目录
    info!("write file target dir: {}", target_dir.display());
    // 创建目录
    if let Err(e) = tokio::fs::create_dir_all(&target_dir).await {
        return Err(ApiError::with_inner_error(e.to_string()));
    };
    // 写入数据
    let file_path = target_dir.join(filename);
    if let Err(e) = tokio::fs::write(&file_path, data).await {
        return Err(ApiError::with_inner_error(e.to_string()));
    };
    Ok(())
}

/// 删除文件
pub async fn delete_file(target_dir: &Path, filename: &str) -> Result<(), ApiError> {
    let file_path = target_dir.join(filename);
    if let Err(e) = tokio::fs::remove_file(&file_path).await {
        return Err(ApiError::with_inner_error(e.to_string()));
    };
    Ok(())
}
