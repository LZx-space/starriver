use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_session::SessionExt;
use actix_web::dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::{Method, StatusCode};
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::infrastructure::model::err::CodedErr;
use crate::infrastructure::security::authentication::core::authenticator::Authenticator;
use crate::infrastructure::security::authentication::user_principal::{
    UserAuthenticator, UserProof, UserRepository,
};
use crate::infrastructure::security::authentication::web::actix::error::ErrUnauthorized;

pub struct AuthenticationService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthenticationService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            if req.uri().path().eq("/login") && req.method().eq(&Method::POST) {
                let form_login_cmd = extract_params(&mut req).await;
                return match form_login_cmd {
                    Ok(cmd) => {
                        let proof = UserProof::new(cmd.username, cmd.password);
                        let repository = UserRepository {};
                        let authenticator = UserAuthenticator::new(repository);
                        match authenticator.authenticate(&proof) {
                            Ok(principal) => {
                                req.get_session()
                                    .insert("authenticated_principal".to_string(), principal)
                                    .expect("TODO: panic message");
                                Ok(ServiceResponse::new(
                                    req.request().to_owned(),
                                    HttpResponse::new(StatusCode::OK),
                                ))
                            }
                            Err(e) => {
                                let err = CodedErr::new("A00001".to_string(), e.to_string());
                                let status_code = err.determine_http_status();
                                Ok(ServiceResponse::new(
                                    req.request().to_owned(),
                                    HttpResponse::new(status_code),
                                ))
                            }
                        }
                    }
                    Err(err) => Ok(ServiceResponse::from_err(err, req.request().to_owned())),
                };
            }
            if req.cookie("id").is_none() {
                return Ok(ServiceResponse::from_err(
                    ErrUnauthorized {},
                    req.request().to_owned(),
                ));
            }
            service.call(req).await
        })
    }
}

async fn extract_params(req: &mut ServiceRequest) -> Result<FormLoginCmd, Error> {
    let http_req = req.request().clone();
    let payload = &mut req.take_payload();
    FormLoginCmd::from_request(&http_req, payload).await
}

pub struct AuthenticationTransform {}

impl<S> Transform<S, ServiceRequest> for AuthenticationTransform
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Transform = AuthenticationService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationService {
            service: Rc::new(service),
        }))
    }
}

#[derive(Deserialize)]
pub struct FormLoginCmd {
    pub username: String,
    pub password: String,
}

impl FromRequest for FormLoginCmd {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        ready(Ok(FormLoginCmd {
            username: "LZx".to_string(),
            password: "password".to_string(),
        }))
    }
}
