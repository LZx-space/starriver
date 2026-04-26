use crate::config::app_state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use starriver_application::article_dto::req::{ArticleAttachmentCmd, PageQuery, UpdateArticleCmd};
use starriver_infrastructure::error::ApiError;
use starriver_infrastructure::extract::{Json, Multipart, Path, Query};
use starriver_infrastructure::security::authentication::_default_impl::AuthenticatedUser;
use starriver_infrastructure::util::file_utils::get_extension;
use tracing::info;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn paginate(
    state: State<AppState>,
    query: Query<PageQuery>,
) -> Result<impl IntoResponse, ApiError> {
    state.article_application.paginate(query.0).await.map(Json)
}

pub async fn show(state: State<AppState>, id: Path<Uuid>) -> Result<impl IntoResponse, ApiError> {
    state.article_application.find(id.0).await.map(Json)
}

pub async fn create_draft(
    state: State<AppState>,
    user: AuthenticatedUser,
) -> Result<impl IntoResponse, ApiError> {
    state.article_application.create_draft(user).await.map(Json)
}

pub async fn update(
    state: State<AppState>,
    id: Path<Uuid>,
    user: AuthenticatedUser,
    cmd: Json<UpdateArticleCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .article_application
        .update(user, id.0, cmd.0)
        .await
        .map(Json)
}

pub async fn upload_attachment(
    state: State<AppState>,
    id: Path<Uuid>,
    user: AuthenticatedUser,
    mut file: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    while let Ok(Some(field)) = file.0.next_field().await {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        info!(
            user_id = %user.id,
            file_name = %file_name,
            article_id = %id.0,
            "uploading file attachment"
        );
        // 获取文件格式（从文件名中提取）
        let extension = get_extension(file_name.as_str());

        // 获取文件数据（对于大文件，建议使用 field.bytes_stream() 进行流式处理）
        let data = match field.bytes().await {
            Ok(d) => d,
            Err(_) => continue,
        };

        let file = ArticleAttachmentCmd {
            extension: extension.to_string(),
            data,
        };
        let attachment = state
            .article_application
            .upload_attachment(user, id.0, file)
            .await?;
        return Ok(Json::from(attachment));
    }
    Err(ApiError::with_bad_request("no file uploaded"))
}

pub async fn delete(
    state: State<AppState>,
    id: Path<Uuid>,
    user: AuthenticatedUser,
) -> Result<impl IntoResponse, ApiError> {
    state
        .article_application
        .delete_by_id(user, id.0)
        .await
        .map(Json)
}
