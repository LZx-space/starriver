use axum::{extract::State, response::IntoResponse};
use starriver_shared_base::dto::PageQuery;
use starriver_shared_framework::{
    extract::{Json, Query},
    response::ApiError,
};

use crate::{error_mapping::map_error, port_in::state::IdentityState};

pub async fn paginate(
    state: State<IdentityState>,
    q: Query<PageQuery>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .security_event_interactor
        .paginate(q.0)
        .await
        .map_err(map_error)
        .map(Json)
}
