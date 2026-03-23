use crate::config::app_state::AppState;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use starriver_application::user_dto::{EmailVerifyCmd, UserCmd};
use starriver_infrastructure::security::authentication::_default_impl::AuthenticatedUser;

pub async fn me(user: AuthenticatedUser) -> impl IntoResponse {
    Json(user)
}

pub async fn register_inactive_user(
    state: State<AppState>,
    cmd: Json<UserCmd>,
) -> impl IntoResponse {
    let cmd = cmd.0;
    state.user_application.register_user(cmd).await
}

pub async fn verify_email(state: State<AppState>, cmd: Json<EmailVerifyCmd>) -> impl IntoResponse {
    state.user_application.send_verification_email(cmd.0).await
}
