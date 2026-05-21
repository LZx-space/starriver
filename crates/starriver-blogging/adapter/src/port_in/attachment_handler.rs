use std::path::Path;

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use starriver_shared_base::dto::IdValue;
use starriver_shared_framework::{
    extract::{Json, Multipart},
    io::{MultipartFieldAsyncReader, TokioFileAsyncWriter},
    middleware::authentication::default_impl::AuthenticatedUser,
    response::ApiError,
};
use tracing::info;
use uuid::Uuid;

use crate::port_in::state::BloggingState;

#[axum::debug_handler]
pub async fn upload_attachment(
    state: State<BloggingState>,
    _: AuthenticatedUser,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let mut urls = Vec::new();
    while let Some(mut field) = multipart
        .0
        .next_field()
        .await
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let file_name = field.file_name().unwrap_or_default();
        info!(field=%file_name, "processing field");
        if file_name.is_empty() {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "file name is empty".to_string(),
            ));
        }

        let claimed_mime_type = field.content_type().map(|s| s.to_owned()).ok_or_else(|| {
            ApiError::new(
                StatusCode::BAD_REQUEST,
                "No content type provided".to_string(),
            )
        })?;

        // 创建文件
        let upload_cfg = &state.uploads;
        let extension = Path::new(file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default();
        let safe_name = format!("{}.{}", Uuid::now_v7(), extension);
        let save_path = format!("{}/{}", upload_cfg.storage_dir, safe_name);
        info!(field=%file_name, "new async writer");
        let async_writer = TokioFileAsyncWriter::new(save_path)
            .await
            .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;
        info!(field=%file_name, "new async reader");
        let async_reader = MultipartFieldAsyncReader::new(&mut field);

        let id = state
            .attachment_service
            .upload(claimed_mime_type.as_str(), async_reader, async_writer)
            .await
            .map(|e| e.id().to_owned())
            .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        urls.push(IdValue {
            id,
            value: format!("{}/{}", upload_cfg.proxy_prefix, safe_name),
        });
    }
    Ok(Json(urls))
}
