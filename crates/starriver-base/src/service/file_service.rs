use std::path::{Path, PathBuf};

use axum::extract::Multipart;
use bytes::Bytes;
use infer::MatcherType;
use tokio::io::AsyncWriteExt;
use tracing::debug;
use uuid::Uuid;

use crate::{error::ApiError, service::config_service::Uploads, util::file_utils::get_extension};

pub async fn storage(
    mut multipart: Multipart,
    upload_cfg: &Uploads,
    allowed_types: Vec<MatcherType>,
) -> Result<Vec<PathBuf>, ApiError> {
    const INFER_MAGIC_BUF_MAX_LEN: usize = 256; // 目前实际最大100字节
    const INFER_MAGIC_BUF_MIN_LEN: usize = 1; // exe只需要1字节

    let mut saved_paths = Vec::new();
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::with_bad_request(e.to_string()))?
    {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let extension = get_extension(file_name.as_str());
        // 创建文件
        let safe_name = format!("{}.{}", Uuid::now_v7(), extension);
        let save_path = format!("{}/{}", upload_cfg.storage_dir, safe_name);

        let mut file = tokio::fs::File::create(&save_path)
            .await
            .map_err(|e| ApiError::with_bad_request(e.to_string()))?;

        let mut magic_buf = Vec::new();
        let mut type_checked = false;

        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|e| ApiError::with_bad_request(e.to_string()))?
        {
            // 累积前导字节用于类型检测（只在第一次检测前收集）
            if !type_checked && magic_buf.len() < INFER_MAGIC_BUF_MAX_LEN {
                let remaining = INFER_MAGIC_BUF_MAX_LEN - magic_buf.len();
                let take = chunk.len().min(remaining);
                magic_buf.extend_from_slice(&chunk[..take]);
            }
            if !type_checked && magic_buf.len() > INFER_MAGIC_BUF_MIN_LEN {
                if let Some(file_type) = infer::get(&magic_buf) {
                    if !allowed_types.contains(&file_type.matcher_type()) {
                        return Err(ApiError::with_bad_request(format!(
                            "disallowed file type, expected one of {:?}, actual {:?}",
                            allowed_types,
                            file_type.matcher_type()
                        )));
                    }
                    if file_type.extension() != extension {
                        // 删除已创建的文件
                        if let Err(e) = tokio::fs::remove_file(&save_path).await {
                            return Err(ApiError::with_bad_request(format!(
                                "failed to remove file after type mismatch: {}",
                                e
                            )));
                        };
                        return Err(ApiError::with_bad_request(format!(
                            "file type mismatch, expected {}, actual {}",
                            extension,
                            file_type.extension()
                        )));
                    }
                    type_checked = true;
                } else if magic_buf.len() >= INFER_MAGIC_BUF_MAX_LEN {
                    return Err(ApiError::with_bad_request("cant't infer file type"));
                }
            }
            file.write_all(&chunk)
                .await
                .map_err(|e| ApiError::with_inner_error(e.to_string()))?;
        }
        saved_paths.push(save_path.into());
    }
    Ok(saved_paths)
}

/// 将数据写入文件
pub async fn write_to_file(target_dir: &Path, filename: &str, data: Bytes) -> Result<(), ApiError> {
    // 保存目录
    debug!(dir = %target_dir.display(), "writing file to target directory");
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
pub async fn delete_file(file_path: &Path) -> Result<(), ApiError> {
    if let Err(e) = tokio::fs::remove_file(file_path).await {
        return Err(ApiError::with_inner_error(e.to_string()));
    };
    Ok(())
}
