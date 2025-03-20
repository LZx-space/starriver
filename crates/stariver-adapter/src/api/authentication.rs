use actix_web::web::Json;
use actix_web::{Responder, get};

use crate::user_principal::User;
use stariver_infrastructure::security::authentication::core::principal::Principal;

#[get("/session/user")]
pub async fn validate_authenticated(user: User) -> impl Responder {
    let string = user.id();
    Json(String::from(string).clone().to_string())
}
