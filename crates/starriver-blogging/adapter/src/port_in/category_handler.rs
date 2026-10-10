use axum::{extract::State, response::IntoResponse};
use starriver_blogging_application::dto::category_dto::req::CreateOrUpdateCategoryCmd;
use starriver_shared_infra::{
    extract::{Json, Path},
    middleware::authentication::default_impl::AuthenticatedJwtClaims,
};
use uuid::Uuid;

use crate::{api_error::ApiError, port_in::state::BloggingState};

pub async fn list_all(state: State<BloggingState>) -> Result<impl IntoResponse, ApiError> {
    let categories = state.category_interactor.list_all().await?;
    Ok(Json(categories))
}

pub async fn show(
    state: State<BloggingState>,
    id: Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let category = state.category_interactor.find(id.0).await?;
    Ok(Json(category))
}

pub async fn create(
    state: State<BloggingState>,
    user: AuthenticatedJwtClaims,
    cmd: Json<CreateOrUpdateCategoryCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let created = state
        .category_interactor
        .create(user.into(), cmd.0.name)
        .await?;
    Ok(Json(created))
}

pub async fn update(
    state: State<BloggingState>,
    user: AuthenticatedJwtClaims,
    Path(id): Path<Uuid>,
    Json(cmd): Json<CreateOrUpdateCategoryCmd>,
) -> Result<impl IntoResponse, ApiError> {
    let updated = state
        .category_interactor
        .update(user.into(), id, cmd.name)
        .await?;
    Ok(Json(updated))
}

pub async fn delete(
    state: State<BloggingState>,
    user: AuthenticatedJwtClaims,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state.category_interactor.delete(user.into(), id).await?;
    Ok(Json(()))
}
