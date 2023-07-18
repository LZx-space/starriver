use actix_session::Session;
use actix_web::web::{Form, Json};
use actix_web::{post, Responder};
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
                .insert("sid".to_string(), principal.credentials().username())
                .expect("TODO: panic message");
            Json("success")
        }
        Err(_) => Json("failure"),
    }
}

#[derive(Deserialize)]
pub struct FormLoginCmd {
    username: String,
    password: String,
}
