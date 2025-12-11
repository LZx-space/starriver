use crate::assembler::blog::{cmd_2_new_entity, cmd_2_update_entity};
use crate::config::app_state::AppState;
use crate::model::blog::{BlogCmd, BlogVo};
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use starriver_infrastructure::model::page::PageQuery;
use uuid::Uuid;

pub async fn page(state: State<AppState>, params: Query<PageQuery>) -> impl IntoResponse {
    let page_query = params.0;
    state
        .blog_application
        .page(page_query)
        .await
        .map(|e| Json(e))
}

pub async fn find_one(state: State<AppState>, id: Path<Uuid>) -> impl IntoResponse {
    state
        .blog_application
        .find_by_id(id.0)
        .await
        .map(|e| {
            e.map(|a| BlogVo {
                title: a.title,
                body: a.body,
                state: a.state.to_string(),
            })
        })
        .map(|e| Json(e))
}

pub async fn insert(state: State<AppState>, cmd: Json<BlogCmd>) -> impl IntoResponse {
    let cmd = cmd.0;
    let blog = cmd_2_new_entity(cmd, "LZx".to_string());
    state.0.blog_application.add(blog).await.map(|e| Json(e))
}

pub async fn update(
    state: State<AppState>,
    id: Path<Uuid>,
    cmd: Json<BlogCmd>,
) -> impl IntoResponse {
    let cmd = cmd.0;
    let id = id.0;
    let to_update = state.blog_application.find_by_id(id).await.expect("");
    if to_update.is_none() {
        ()
    }
    let blog = cmd_2_update_entity(cmd, to_update.unwrap());
    state.blog_application.update(blog).await.map(|e| Json(e))
}

pub async fn delete(state: State<AppState>, id: Path<Uuid>) -> impl IntoResponse {
    state
        .blog_application
        .delete_by_id(id.0)
        .await
        .map(|e| Json(e))
}
