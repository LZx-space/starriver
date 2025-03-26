use actix_web::web::Json;
use actix_web::{Responder, post, web};

use crate::config::app_state::AppState;
use crate::model::user::UserCmd;

#[post("/users")]
pub async fn insert(state: web::Data<AppState>, cmd: Json<UserCmd>) -> impl Responder {
    let cmd = cmd.into_inner();
    state
        .user_application
        .register_user(&cmd.username, &cmd.password)
        .await
        .map(|e| Json(e))
}
