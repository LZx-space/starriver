use crate::config::app_state::AppState;
use crate::model::user::UserCmd;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;

pub async fn insert(state: State<AppState>, cmd: Json<UserCmd>) -> impl IntoResponse {
    let cmd = cmd.0;
    state
        .user_application
        .register_user(&cmd.username, &cmd.password)
        .await
        .map(|e| Json(e))
}
