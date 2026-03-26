use crate::config::app_state::AppState;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_valid::{Valid, ValidEx};
use starriver_application::user_dto::req::{EmailVerifyCmd, UserCmd};
use starriver_infrastructure::security::authentication::_default_impl::AuthenticatedUser;

pub async fn me(user: AuthenticatedUser) -> impl IntoResponse {
    Json(user)
}

pub async fn register_inactive_user(
    state: State<AppState>,
    cmd: ValidEx<Json<UserCmd>>,
) -> impl IntoResponse {
    let cmd = cmd.into_inner().0;
    state.user_application.register_user(cmd).await
}

pub async fn verify_email(
    state: State<AppState>,
    cmd: Valid<Json<EmailVerifyCmd>>,
) -> impl IntoResponse {
    let cmd = cmd.into_inner().0;
    state.user_application.send_verification_email(cmd).await
}
