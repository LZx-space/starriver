use actix_web::web::Json;
use actix_web::{get, Responder};

use stariver_core::infrastructure::security::authentication::core::principal::Principal;
use stariver_core::infrastructure::security::authentication::user_principal::User;

#[get("/session/user")]
pub async fn validate_authenticated(user: User) -> impl Responder {
    let string = user.id();
    Json(String::from(string).clone().to_string())
}