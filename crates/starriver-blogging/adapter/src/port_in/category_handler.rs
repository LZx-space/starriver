use axum::{extract::State, response::IntoResponse};
use starriver_blogging_application::dto::category_dto::req::CreateOrUpdateCategoryCmd;
use starriver_shared_framework::{
    extract::{Json, Path},
    middleware::authentication::default_impl::AuthenticatedUser,
    response::ApiError,
};
use uuid::Uuid;

use crate::{error_mapping::map_error, port_in::state::BloggingState};

pub async fn list_all(state: State<BloggingState>) -> Result<impl IntoResponse, ApiError> {
    state
        .category_interactor
        .list_all()
        .await
        .map(Json)
        .map_err(map_error)
}

pub async fn show(
    state: State<BloggingState>,
    id: Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .category_interactor
        .find(id.0)
        .await
        .map(Json)
        .map_err(map_error)
}

pub async fn create(
    state: State<BloggingState>,
    user: AuthenticatedUser,
    cmd: Json<CreateOrUpdateCategoryCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .category_interactor
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
) -> Result<impl IntoResponse, ApiError> {
    state
        .category_interactor
        .update(user.0, id, cmd.name)
        .await
        .map(Json)
        .map_err(map_error)
}

pub async fn delete(
    state: State<BloggingState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .category_interactor
        .delete(user.0, id)
        .await
        .map(Json)
        .map_err(map_error)
}
