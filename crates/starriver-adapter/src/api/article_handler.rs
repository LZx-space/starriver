use crate::config::app_state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use starriver_base::dto::article_dto::req::{PageQuery, UpdateArticleCmd};
use starriver_base::error::ApiError;
use starriver_base::extract::{Json, Multipart, Path, Query};
use starriver_base::security::authentication::_default_impl::AuthenticatedUser;
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
    multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    state
        .article_application
        .upload_attachment(user, id.0, multipart)
        .await
        .map(Json::from)
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
