use crate::infrastructure::model::err::CodedErr;
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::web::{Form, Json};
use actix_web::{get, post, Responder};
use serde::Deserialize;

use crate::infrastructure::security::authentication::core::authenticator::Authenticator;
use crate::infrastructure::security::authentication::core::form::{
    UserCredentialsRepository, UsernamePasswordCredentials,
    UsernamePasswordCredentialsAuthenticator,
};
use crate::infrastructure::security::authentication::core::principal::{ClientDetails, Principal};

#[post("/login")]
pub async fn login_in(session: Session, params: Form<FormLoginCmd>) -> impl Responder {
    let login_params = params.into_inner();
    let repository = UserCredentialsRepository {};
    let authenticator = UsernamePasswordCredentialsAuthenticator::new(Box::new(repository));

    let credentials =
        UsernamePasswordCredentials::new(login_params.username, login_params.password);
    let mut principal = Principal::new(credentials, ClientDetails {});
    match authenticator.authenticate(&mut principal) {
        Ok(_) => {
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
    match session.get::<Principal<UsernamePasswordCredentials>>("authenticated_principal") {
        Ok(p) => match p {
            Some(principal) => {
                let c = principal.credentials();
                let x = c.username();
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
