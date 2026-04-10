use crate::config::app_state::AppState;
use axum::Json;
use axum::extract::{Multipart, Path, Query, State};
use axum::response::IntoResponse;
use axum_valid::Valid;
use starriver_application::blog_dto::req::{BlogAttachmentCmd, BlogCmd};
use starriver_infrastructure::error::ApiError;
use starriver_infrastructure::model::page::PageQuery;
use starriver_infrastructure::security::authentication::_default_impl::AuthenticatedUser;
use starriver_infrastructure::util::file_utils::get_extension;
use tracing::info;
use uuid::Uuid;

pub async fn page(
    state: State<AppState>,
    params: Valid<Query<PageQuery>>,
) -> Result<impl IntoResponse, ApiError> {
    let page_query = params.into_inner().0;
    state.blog_application.page(page_query).await.map(Json)
}

pub async fn find_one(
    state: State<AppState>,
    id: Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state.blog_application.find_by_id(id.0).await.map(Json)
}

pub async fn insert_empty_draft(
    state: State<AppState>,
    user: AuthenticatedUser,
) -> Result<impl IntoResponse, ApiError> {
    state.blog_application.add_empty_draft(user).await.map(Json)
}

pub async fn update(
    state: State<AppState>,
    id: Path<Uuid>,
    user: AuthenticatedUser,
    cmd: Json<BlogCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let id = id.0;
    let cmd = cmd.0;
    state.blog_application.update(user, id, cmd).await.map(Json)
}

pub async fn upload_attachment(
    state: State<AppState>,
    id: Path<Uuid>,
    user: AuthenticatedUser,
    mut file: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    while let Ok(Some(field)) = file.next_field().await {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        info!("user [{}] upload file [{}]", user.id, file_name);
        // 获取文件格式（从文件名中提取）
        let extension = get_extension(file_name.as_str());

        // 获取文件数据（对于大文件，建议使用 field.bytes_stream() 进行流式处理）
        let data = match field.bytes().await {
            Ok(d) => d,
            Err(_) => continue,
        };

        let file = BlogAttachmentCmd {
            extension: extension.to_string(),
            data,
        };
        let url = state
            .blog_application
            .upload_attachment(user, id.0, file)
            .await?;
        return Ok(Json::from(url));
    }
    Err(ApiError::with_bad_request("no file uploaded"))
}

pub async fn delete(
    state: State<AppState>,
    id: Path<Uuid>,
    user: AuthenticatedUser,
) -> Result<impl IntoResponse, ApiError> {
    state
        .blog_application
        .delete_by_id(user, id.0)
        .await
        .map(Json)
}
