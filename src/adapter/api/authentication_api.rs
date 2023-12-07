use actix_session::Session;
use actix_web::{get, Responder};
use actix_web::web::Json;

use crate::infrastructure::security::authentication::core::principal::Principal;
use crate::infrastructure::security::authentication::user_principal::User;

#[get("/session/user")]
pub async fn validate_authenticated(session: Session) -> impl Responder {
    match session.get::<User>("authenticated_principal") {
        Ok(p) => match p {
            Some(principal) => {
                let x = principal.id();
                Json(x.clone().to_string())
            }
            None => Json("false".to_string()),
        },
        Err(e) => Json(e.to_string()),
    }
}
