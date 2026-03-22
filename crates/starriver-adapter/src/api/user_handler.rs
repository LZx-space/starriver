use crate::config::app_state::AppState;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use starriver_application::user_dto::UserCmd;

pub async fn register_inactive_user(
    state: State<AppState>,
    cmd: Json<UserCmd>,
) -> impl IntoResponse {
    let cmd = cmd.0;
    state
        .user_application
        .register_inactive_user(&cmd.username, &cmd.password)
        .await
}
