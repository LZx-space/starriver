use axum::{extract::State, response::IntoResponse};
use starriver_blogging_application::dto::post_dto::req::{
    PageQuery, SaveOrUpdatePostCmd, SearchQuery,
};
use starriver_shared_framework::{
    extract::{Json, Path, Query},
    middleware::authentication::default_impl::AuthenticatedUser,
    response::ApiError,
};
use uuid::Uuid;

use crate::{error_mapping::map_error, port_in::state::BloggingState};

pub async fn paginate(
    state: State<BloggingState>,
    query: Query<PageQuery>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .post_interactor
        .paginate(query.0)
        .await
        .map_err(map_error)
        .map(Json)
}

pub async fn search(
    state: State<BloggingState>,
    query: Query<SearchQuery>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .post_interactor
        .search(&query.0.q)
        .await
        .map_err(map_error)
        .map(Json)
}

pub async fn show(
    state: State<BloggingState>,
    id: Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .post_interactor
        .find(id.0)
        .await
        .map_err(map_error)
        .map(Json)
}

pub async fn create(
    state: State<BloggingState>,
    user: AuthenticatedUser,
    cmd: Json<SaveOrUpdatePostCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .post_interactor
        .create(user.0, cmd.0)
        .await
        .map_err(map_error)
        .map(Json)
}

pub async fn update(
    state: State<BloggingState>,
    id: Path<Uuid>,
    user: AuthenticatedUser,
    cmd: Json<SaveOrUpdatePostCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .post_interactor
        .update(user.0, id.0, cmd.0)
        .await
        .map_err(map_error)
        .map(Json)
}

pub async fn delete(
    state: State<BloggingState>,
    id: Path<Uuid>,
    user: AuthenticatedUser,
) -> Result<impl IntoResponse, ApiError> {
    state
        .post_interactor
        .delete_by_id(user.0, id.0)
        .await
        .map_err(map_error)
        .map(Json)
}
