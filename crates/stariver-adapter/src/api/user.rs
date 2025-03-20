use actix_web::web::Json;
use actix_web::{Responder, post, web};

use crate::app_state::AppState;
use crate::model::user::UserCmd;
use stariver_application::service::user_service::UserApplication;
use stariver_domain::user::aggregate::User;

#[post("/users")]
pub async fn insert(state: web::Data<AppState>, cmd: Json<UserCmd>) -> impl Responder {
    let application = UserApplication::new(state.conn);
    let cmd = cmd.into_inner();
    application
        .insert(User::new_with_username_and_password(
            cmd.username.as_str(),
            cmd.password.as_str(),
        ))
        .await
        .map(|e| Json(e))
}
