use std::future::{Future, ready, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_session::SessionExt;
use actix_web::{Error, FromRequest, HttpMessage, HttpResponse};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::{Method, StatusCode};
use actix_web::web::Form;
use serde::Deserialize;

use crate::infrastructure::model::err::CodedErr;
use crate::infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use crate::infrastructure::security::authentication::user_principal::{
    User, UserAuthenticator, UserProof, UserRepository,
};
use crate::infrastructure::security::authentication::web::actix::error::ErrUnauthorized;

pub struct AuthenticationService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthenticationService<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse, Error=Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            if is_login_request(&mut req) {
                let form_login_cmd = extract_params(&mut req).await;
                return match form_login_cmd {
                    Ok(cmd) => {
                        let proof = UserProof::new(cmd.username.clone(), cmd.password.clone());
                        let repository = UserRepository {};
                        let authenticator = UserAuthenticator::new(repository);
                        match authenticator.authenticate(&proof) {
                            Ok(principal) => success_handle(&mut req, principal),
                            Err(e) => failure_handle(&mut req, e),
                        }
                    }
                    Err(err) => Err(err),
                };
            }
            if !is_principal_authenticated(&mut req) {
                return Ok(ServiceResponse::from_err(
                    ErrUnauthorized {},
                    req.request().to_owned(),
                ));
            }
            service.call(req).await
        })
    }
}

pub struct AuthenticationTransform {}

impl<S> Transform<S, ServiceRequest> for AuthenticationTransform
    where
        S: Service<ServiceRequest, Response=ServiceResponse, Error=Error> + 'static,
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

fn is_login_request(req: &mut ServiceRequest) -> bool {
    req.uri().path().eq("/login") && req.method().eq(&Method::POST)
}

async fn extract_params(req: &mut ServiceRequest) -> Result<Form<FormLoginCmd>, Error> {
    let http_req = req.request().clone();
    let payload = &mut req.take_payload();
    Form::<FormLoginCmd>::from_request(&http_req, payload).await
}

fn is_principal_authenticated(req: &mut ServiceRequest) -> bool {
    req.cookie("id").is_some()
}

fn success_handle(req: &mut ServiceRequest, principal: User) -> Result<ServiceResponse, Error> {
    return match req.get_session()
        .insert("authenticated_principal".to_string(), principal)
        .map_err(|e| { Error::from(e) }) {
        Ok(_) => {
            Ok(ServiceResponse::new(
                req.request().to_owned(),
                HttpResponse::new(StatusCode::OK),
            ))
        }
        Err(e) => {
            Err(Error::from(e))
        }
    };
}

fn failure_handle(
    req: &mut ServiceRequest,
    e: AuthenticationError,
) -> Result<ServiceResponse, Error> {
    let err = CodedErr::new("A00001".to_string(), e.to_string());
    let status_code = err.determine_http_status();
    Ok(ServiceResponse::new(
        req.request().to_owned(),
        HttpResponse::new(status_code),
    ))
}
