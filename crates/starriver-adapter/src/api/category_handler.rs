use axum::{extract::State, response::IntoResponse};
use starriver_application::category_dto::req::CreateOrUpdateCategoryCmd;
use starriver_infrastructure::{
    error::ApiError,
    extract::{Json, Path},
    security::authentication::_default_impl::AuthenticatedUser,
};
use uuid::Uuid;

use crate::config::app_state::AppState;

pub async fn list_all(state: State<AppState>) -> Result<impl IntoResponse, ApiError> {
    state.category_application.list().await.map(Json)
}

pub async fn create(
    state: State<AppState>,
    user: AuthenticatedUser,
    cmd: Json<CreateOrUpdateCategoryCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .category_application
        .insert(user, cmd.0.name)
        .await
        .map(Json)
}

pub async fn update(
    state: State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(cmd): Json<CreateOrUpdateCategoryCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .category_application
        .update(user, id, cmd.name)
        .await
        .map(Json)
}

pub async fn delete(
    state: State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state.category_application.delete(user, id).await.map(Json)
}
