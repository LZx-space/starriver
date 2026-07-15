use std::path::Path;

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use starriver_blogging_domain::attachment::{entity::Attachment, value_object::Extension};
use starriver_shared_base::upload_file::UploadLocationResolver;
use starriver_shared_framework::{
    extract::{Json, Multipart},
    io::{MultipartFieldAsyncReader, TokioFileAsyncWriter},
    middleware::authentication::default_impl::AuthenticatedUser,
    response::ApiError,
};
use tracing::{error, info};
use uuid::Uuid;

use crate::port_in::state::BloggingState;

#[axum::debug_handler]
pub async fn upload_attachment(
    state: State<BloggingState>,
    _: AuthenticatedUser,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let mut attachments = Vec::new();
    while let Some(mut field) = multipart
        .0
        .next_field()
        .await
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let file_name = field.file_name().unwrap_or_default();
        info!(filename=%file_name, "processing field");

        // create file; bad code, but DIP
        let claimed_extension = Extension::new(
            Path::new(file_name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or_default(),
        )
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;
        info!(claimed_extension=%claimed_extension.as_str());

        let attachment_id = Uuid::now_v7(); // 附件ID生成附件名
        let attachment_name = Attachment::make_file_name(&attachment_id, &claimed_extension);
        let save_path = state
            .upload_file_url_builder
            .save_path(attachment_name.as_str());

        info!("new async writer");
        let async_writer = TokioFileAsyncWriter::new(&save_path)
            .await
            .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

        info!("new async reader");
        let async_reader = MultipartFieldAsyncReader::new(&mut field);

        info!("uploading attachment");
        match state
            .attachment_interactor
            .upload(
                attachment_id, // 附件ID生成附件名，确保外部Writer的文件和附件是同一个
                claimed_extension.as_str(),
                async_reader,
                async_writer,
            )
            .await
            .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        {
            Ok(attachment) => {
                attachments.push(attachment);
            }
            Err(err) => {
                if let Err(rm_err) = tokio::fs::remove_file(&save_path).await {
                    error!(error=%rm_err, "remove file error");
                }
                return Err(err);
            }
        };
    }
    Ok(Json(attachments))
}
