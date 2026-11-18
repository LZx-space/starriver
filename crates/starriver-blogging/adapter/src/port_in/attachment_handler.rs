use axum::{extract::State, response::IntoResponse};
use starriver_shared_infra::{
    extract::{Json, Multipart},
    io::MultipartFieldAsyncReader,
    middleware::authentication::default_impl::AuthenticatedJwtClaims,
};
use tracing::info;

use crate::api_error::ApiError;
use crate::port_in::state::BloggingState;

pub async fn upload(
    state: State<BloggingState>,
    user: AuthenticatedJwtClaims,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    info!(username = %&user.username , "upload attachment");
    let mut attachments = Vec::new();
    while let Some(mut field) = multipart.0.next_field().await? {
        info!("upload attachment, new async reader");
        let async_reader = MultipartFieldAsyncReader::new(&mut field);
        match state.attachment_interactor.upload(async_reader).await {
            Ok(attachment) => {
                attachments.push(attachment);
            }
            Err(err) => {
                return Err(err.into());
            }
        };
    }
    Ok(Json(attachments))
}
