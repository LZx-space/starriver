use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::web::{Form, Json};
use actix_web::{get, post, Responder};
use serde::Deserialize;

use crate::infrastructure::model::err::CodedErr;
use crate::infrastructure::security::authentication::core::authenticator::Authenticator;
use crate::infrastructure::security::authentication::core::principal::Principal;
use crate::infrastructure::security::authentication::user_principal::{
    User, UserAuthenticator, UserProof, UserRepository,
};

#[post("/login")]
pub async fn login_in(session: Session, params: Form<FormLoginCmd>) -> impl Responder {
    let login_params = params.into_inner();
    let proof = UserProof::new(login_params.username, login_params.password);
    let repository = UserRepository {};
    let authenticator = UserAuthenticator::new(repository);
    match authenticator.authenticate(&proof) {
        Ok(principal) => {
            session
                .insert("authenticated_principal".to_string(), principal)
                .expect("TODO: panic message");
            (Json("success".to_string()), StatusCode::OK)
        }
        Err(e) => {
            let err = CodedErr::new("A00001".to_string(), e.to_string());
            let status_code = err.determine_http_status();
            (Json(err.to_string()), status_code)
        }
    }
}

#[get("/auth")]
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

#[derive(Deserialize)]
pub struct FormLoginCmd {
    username: String,
    password: String,
}
