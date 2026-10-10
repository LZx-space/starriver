use axum::{extract::State, response::IntoResponse};
use starriver_shared_base::dto::PageQuery;
use starriver_shared_infra::{
    extract::{Json, Query},
    middleware::authentication::default_impl::AuthenticatedJwtClaims,
};

use crate::{api_error::ApiError, port_in::state::IdentityState};

pub async fn paginate(
    state: State<IdentityState>,
    _: AuthenticatedJwtClaims,
    q: Query<PageQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let page = state.security_event_interactor.paginate(q.0).await?;
    Ok(Json(page))
}
