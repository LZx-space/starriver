use actix_web::web::Json;
use actix_web::{Responder, get};

use crate::config::user_principal::User;
use starriver_infrastructure::security::authentication::core::principal::Principal;

#[get("/session/user")]
pub async fn validate_authenticated(user: User) -> impl Responder {
    let username = user.id().as_str();
    Json(String::from(username))
}
