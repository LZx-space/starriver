use actix_web::web::Json;
use actix_web::{post, web, Responder};
use uuid::Uuid;

use stariver_core::application::user_service::UserApplication;
use stariver_core::domain::user::aggregate::User;
use stariver_core::infrastructure::web::app_state::AppState;

use crate::model::user::UserCmd;

#[post("/users")]
pub async fn insert(state: web::Data<AppState>, cmd: Json<UserCmd>) -> impl Responder {
    let application = UserApplication::new(state.conn);
    let cmd = cmd.into_inner();
    application
        .insert(User {
            id: Uuid::now_v7(),
            username: cmd.username,
            password: cmd.password,
        })
        .await
        .map(|e| Json(e))
}
