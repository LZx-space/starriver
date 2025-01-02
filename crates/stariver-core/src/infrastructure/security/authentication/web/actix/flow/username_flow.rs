use actix_session::SessionExt;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::{Method, StatusCode};
use actix_web::web::Form;
use actix_web::{FromRequest, HttpMessage, HttpResponse};
use serde::Deserialize;

use crate::infrastructure::model::err::CodedErr;
use crate::infrastructure::security::authentication::core::authenticator::AuthenticationError;
use crate::infrastructure::security::authentication::user_principal::{
    User, UserAuthenticator, UsernamePasswordCredential,
};
use crate::infrastructure::security::authentication::web::actix::error::ErrUnauthorized;
use crate::infrastructure::security::authentication::web::flow::AuthenticationFlow;

pub struct UsernameFlow {}

impl AuthenticationFlow for UsernameFlow {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Credential = UsernamePasswordCredential;
    type Principal = User;
    type Authenticator = UserAuthenticator;

    fn is_authenticated(&self, req: &Self::Request) -> bool {
        req.cookie("id").is_some()
    }

    fn is_authenticate_request(&self, req: &Self::Request) -> bool {
        req.uri().path().eq("/login") && req.method().eq(&Method::POST)
    }

    async fn extract_credential(
        &self,
        req: &mut Self::Request,
    ) -> Result<UsernamePasswordCredential, AuthenticationError> {
        let http_req = req.request().clone();
        let payload = &mut req.take_payload();
        Form::<FormLoginCmd>::from_request(&http_req, payload)
            .await
            .map(|e| e.into_inner())
            .map_err(|e| AuthenticationError::Unknown)
            .and_then(|e| UsernamePasswordCredential::new(e.username, e.password))
    }

    async fn on_unauthenticated(
        &self,
        req: &Self::Request,
    ) -> Result<Self::Response, AuthenticationError> {
        Ok(ServiceResponse::from_err(
            ErrUnauthorized {},
            req.request().to_owned(),
        ))
    }

    async fn on_authenticate_success(
        &self,
        req: &Self::Request,
        principal: User,
    ) -> Result<Self::Response, AuthenticationError> {
        req.get_session()
            .insert("authenticated_principal".to_string(), principal)
            .map_err(|e| AuthenticationError::UsernameNotFound)?;
        Ok(ServiceResponse::new(
            req.request().to_owned(),
            HttpResponse::new(StatusCode::OK),
        ))
    }

    async fn on_authenticate_failure(
        &self,
        req: &Self::Request,
        e: AuthenticationError,
    ) -> Result<Self::Response, AuthenticationError> {
        let err = CodedErr::new("A00001".to_string(), e.to_string());
        Ok(ServiceResponse::from_err(err, req.request().to_owned()))
    }
}

#[derive(Deserialize, Debug)]
pub struct FormLoginCmd {
    pub username: String,
    pub password: String,
}
