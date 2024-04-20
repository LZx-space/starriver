use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_session::SessionExt;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::{Method, StatusCode};
use actix_web::web::Form;
use actix_web::{Error, FromRequest, HttpMessage, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::infrastructure::model::err::CodedErr;
use crate::infrastructure::repository::user::user_repository::UserRepositoryImpl as DomainUserRepo;
use crate::infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use crate::infrastructure::security::authentication::user_principal::{
    User, UserAuthenticator, UserCredential, UserRepositoryImpl,
};
use crate::infrastructure::security::authentication::web::actix::error::ErrUnauthorized;

pub struct AuthenticationService<S> {
    service: Rc<S>,
    conn: &'static DatabaseConnection,
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
        let conn = self.conn;
        Box::pin(async move {
            if is_login_request(&req) {
                let form_login_cmd = extract_params(&mut req).await?;
                let credential = UserCredential::new(
                    form_login_cmd.username.clone(),
                    form_login_cmd.password.clone(),
                );
                let repository = UserRepositoryImpl {
                    delegate: DomainUserRepo { conn },
                };
                let authenticator = UserAuthenticator::new(repository);
                return match authenticator.authenticate(&credential).await {
                    Ok(principal) => success_handle(&req, principal).await,
                    Err(e) => failure_handle(&req, e).await,
                };
            }
            if !is_principal_authenticated(&req) {
                return un_authenticated_handle(&req).await;
            }
            service.call(req).await
        })
    }
}

pub struct AuthenticationTransform {
    pub conn: &'static DatabaseConnection,
}

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
            conn: self.conn,
        }))
    }
}

#[derive(Deserialize)]
pub struct FormLoginCmd {
    pub username: String,
    pub password: String,
}

fn is_login_request(req: &ServiceRequest) -> bool {
    req.uri().path().eq("/login") && req.method().eq(&Method::POST)
}

async fn extract_params(req: &mut ServiceRequest) -> Result<Form<FormLoginCmd>, Error> {
    let http_req = req.request().clone();
    let payload = &mut req.take_payload();
    Form::<FormLoginCmd>::from_request(&http_req, payload).await
}

fn is_principal_authenticated(req: &ServiceRequest) -> bool {
    req.cookie("id").is_some()
}

async fn un_authenticated_handle(req: &ServiceRequest) -> Result<ServiceResponse, Error> {
    Ok(ServiceResponse::from_err(
        ErrUnauthorized {},
        req.request().to_owned(),
    ))
}

async fn success_handle(req: &ServiceRequest, principal: User) -> Result<ServiceResponse, Error> {
    req.get_session()
        .insert("authenticated_principal".to_string(), principal)
        .map_err(|e| Error::from(e))?;
    Ok(ServiceResponse::new(
        req.request().to_owned(),
        HttpResponse::new(StatusCode::OK),
    ))
}

async fn failure_handle(
    req: &ServiceRequest,
    e: AuthenticationError,
) -> Result<ServiceResponse, Error> {
    let err = CodedErr::new("A00001".to_string(), e.to_string());
    let status_code = err.determine_http_status();
    Ok(ServiceResponse::new(
        req.request().to_owned(),
        HttpResponse::new(status_code),
    ))
}
