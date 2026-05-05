use axum::response::IntoResponse;
use axum::{Json, extract::State};

use starriver_identity_application::dto::user_dto::req::{EmailVerifyCmd, UserCmd};

use crate::port_in::state::IdentityState;
use crate::port_in::user_dto::{ApiError, AuthenticatedUser};

#[axum::debug_handler]
pub async fn me(_: State<IdentityState>, user: AuthenticatedUser) -> impl IntoResponse {
    Json(user)
}

#[axum::debug_handler]
pub async fn register_inactive_user(
    state: State<IdentityState>,
    cmd: Json<UserCmd>,
) -> impl IntoResponse {
    let cmd = cmd.0;
    state
        .user_service
        .register_user(cmd)
        .await
        .map_err(ApiError::from)
}

pub async fn verify_email(
    state: State<IdentityState>,
    cmd: Json<EmailVerifyCmd>,
) -> impl IntoResponse {
    state.user_service.send_verification_email(cmd.0).await
}
