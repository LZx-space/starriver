use axum::{extract::State, response::IntoResponse};
use starriver_shared_framework::{
    extract::{Json, Path},
    middleware::authentication::default_impl::AuthenticatedUser,
};
use uuid::Uuid;

use crate::{
    error_mapping::map_error,
    port_in::{category_dto::req::CreateOrUpdateCategoryCmd, state::BloggingState},
};

pub async fn list_all(state: State<BloggingState>) -> impl IntoResponse {
    state
        .category_service
        .list_all()
        .await
        .map(Json)
        .map_err(map_error)
}

pub async fn show(state: State<BloggingState>, id: Path<Uuid>) -> impl IntoResponse {
    state
        .category_service
        .find(id.0)
        .await
        .map(Json)
        .map_err(map_error)
}

pub async fn create(
    state: State<BloggingState>,
    user: AuthenticatedUser,
    cmd: Json<CreateOrUpdateCategoryCmd>,
) -> impl IntoResponse {
    state
        .category_service
        .create(user.0, cmd.0.name)
        .await
        .map(Json)
        .map_err(map_error)
}

pub async fn update(
    state: State<BloggingState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(cmd): Json<CreateOrUpdateCategoryCmd>,
) -> impl IntoResponse {
    state
        .category_service
        .update(user.0, id, cmd.name)
        .await
        .map(Json)
        .map_err(map_error)
}

pub async fn delete(
    state: State<BloggingState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    state
        .category_service
        .delete(user.0, id)
        .await
        .map(Json)
        .map_err(map_error)
}
