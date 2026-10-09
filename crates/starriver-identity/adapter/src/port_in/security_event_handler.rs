use axum::{extract::State, response::IntoResponse};
use starriver_shared_base::dto::PageQuery;
use starriver_shared_framework::{
    extract::{Json, Query},
    middleware::authentication::default_impl::AuthenticatedJwtClaims,
    response::ApiError,
};

use crate::{error_mapping::map_error, port_in::state::IdentityState};

pub async fn paginate(
    state: State<IdentityState>,
    _: AuthenticatedJwtClaims,
    q: Query<PageQuery>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .security_event_interactor
        .paginate(q.0)
        .await
        .map_err(map_error)
        .map(Json)
}
