use actix_web::web::Json;
use actix_web::{post, web, Responder};

use stariver_core::infrastructure::web::app_state::AppState;

#[post("/users")]
pub async fn insert(state: web::Data<AppState>) -> impl Responder {
    Json("todo")
}
