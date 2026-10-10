use axum::{extract::State, response::IntoResponse};
use starriver_blogging_application::dto::post_dto::req::{PageQuery, SaveOrUpdatePostCmd};
use starriver_shared_base::dto::PageSearch;
use starriver_shared_framework::{
    extract::{Json, Path, Query},
    middleware::authentication::default_impl::AuthenticatedJwtClaims,
};
use uuid::Uuid;

use crate::{api_error::ApiError, port_in::state::BloggingState};

pub async fn paginate(
    state: State<BloggingState>,
    query: Query<PageQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let page = state.post_interactor.paginate(query.0).await?;
    Ok(Json(page))
}

pub async fn search(
    state: State<BloggingState>,
    query: Query<PageSearch>,
) -> Result<impl IntoResponse, ApiError> {
    let results = state.post_interactor.search(query.0).await?;
    Ok(Json(results))
}

pub async fn show(
    state: State<BloggingState>,
    id: Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let post = state.post_interactor.find(id.0).await?;
    Ok(Json(post))
}

pub async fn create(
    state: State<BloggingState>,
    user: AuthenticatedJwtClaims,
    cmd: Json<SaveOrUpdatePostCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let created = state.post_interactor.create(user.into(), cmd.0).await?;
    Ok(Json(created))
}

pub async fn update(
    state: State<BloggingState>,
    id: Path<Uuid>,
    user: AuthenticatedJwtClaims,
    cmd: Json<SaveOrUpdatePostCmd>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .post_interactor
        .update(user.into(), id.0, cmd.0)
        .await?;
    Ok(Json(()))
}

pub async fn delete(
    state: State<BloggingState>,
    id: Path<Uuid>,
    user: AuthenticatedJwtClaims,
) -> Result<impl IntoResponse, ApiError> {
    let deleted = state
        .post_interactor
        .delete_by_id(user.into(), id.0)
        .await?;
    Ok(Json(deleted))
}
