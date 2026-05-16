use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use starriver_identity_application::dto::user_dto::req::{EmailVerifyCmd, UserCmd};
use starriver_shared_framework::extract::{Json, JsonEx};
use starriver_shared_framework::principal::AuthenticatedUser;
use starriver_shared_framework::response::ApiError;

use crate::port_in::state::IdentityState;

pub async fn me(user: AuthenticatedUser) -> impl IntoResponse {
    Json(user)
}

#[axum::debug_handler]
pub async fn register_user(state: State<IdentityState>, cmd: JsonEx<UserCmd>) -> impl IntoResponse {
    let cmd = cmd.0;
    state
        .user_service
        .register_user(cmd)
        .await
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))
}

pub async fn verify_email(
    state: State<IdentityState>,
    cmd: Json<EmailVerifyCmd>,
) -> impl IntoResponse {
    state.user_service.send_verification_email(cmd.0).await
}
