use crate::config::app_state::AppState;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use starriver_application::blog_dto::BlogCmd;
use starriver_infrastructure::error::error::ApiError;
use starriver_infrastructure::model::page::PageQuery;
use uuid::Uuid;

pub async fn page(
    state: State<AppState>,
    params: Query<PageQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let page_query = params.0;
    state
        .blog_application
        .page(page_query)
        .await
        .map(|e| Json(e))
}

pub async fn find_one(
    state: State<AppState>,
    id: Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .blog_application
        .find_by_id(id.0)
        .await
        .map(|e| Json(e))
}

pub async fn insert(
    state: State<AppState>,
    cmd: Json<BlogCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let cmd = cmd.0;
    state.blog_application.add(cmd).await.map(|e| Json(e))
}

pub async fn update(
    state: State<AppState>,
    id: Path<Uuid>,
    cmd: Json<BlogCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let cmd = cmd.0;
    let id = id.0;
    state
        .blog_application
        .update(id, cmd)
        .await
        .map(|e| Json(e))
}

pub async fn delete(state: State<AppState>, id: Path<Uuid>) -> Result<impl IntoResponse, ApiError> {
    state
        .blog_application
        .delete_by_id(id.0)
        .await
        .map(|e| Json(e))
}
