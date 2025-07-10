use crate::config::user_principal::{User, UserAuthenticator, UsernamePasswordCredential};
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::cookie::Cookie;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::{Method, StatusCode};
use actix_web::web::Form;
use actix_web::{FromRequest, HttpMessage, HttpResponse};
use serde::Deserialize;
use starriver_infrastructure::model::err::CodedErr;
use starriver_infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use starriver_infrastructure::security::authentication::web::actix::error::ErrUnauthorized;
use starriver_infrastructure::security::authentication::web::flow::AuthenticationFlow;
use std::ops::{Add, Not};

pub struct UsernameFlow {}

impl AuthenticationFlow for UsernameFlow {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Credential = UsernamePasswordCredential;
    type Principal = User;
    type Authenticator = UserAuthenticator;

    fn is_authenticate_request(&self, req: &Self::Request) -> bool {
        req.uri().path().eq("/login") && req.method().eq(&Method::POST)
    }

    fn is_access_require_authentication(&self, req: &Self::Request) -> bool {
        req.uri().path().eq("/users").not() && req.method().eq(&Method::POST).not()
    }

    async fn is_authenticated(&self, req: &Self::Request) -> bool {
        req.cookie("id").is_some()
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

    async fn authenticate(
        &self,
        authenticator: &Self::Authenticator,
        credential: &Self::Credential,
    ) -> Result<Self::Principal, AuthenticationError> {
        authenticator.authenticate(credential).await
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
        serde_json::to_string(&principal)
            .map_err(|e| AuthenticationError::Unknown)
            .map(|json| {
                let http_response = HttpResponse::build(StatusCode::OK)
                    .cookie(
                        Cookie::build("id", json)
                            .http_only(true)
                            .expires(OffsetDateTime::now_utc().add(Duration::hours(1)))
                            .secure(false)
                            .finish(),
                    )
                    .finish();
                ServiceResponse::new(req.request().clone(), http_response)
            })
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[actix_web::test]
    async fn test_is_access_require_authentication() {
        let flow = UsernameFlow {};
        let req = TestRequest::default()
            .uri("/not_users")
            .method(Method::GET)
            .to_srv_request();
        assert!(flow.is_access_require_authentication(&req));
    }

    #[actix_web::test]
    async fn test_is_authenticated() {
        let flow = UsernameFlow {};
        let req = TestRequest::default()
            .cookie(Cookie::new("id", "test"))
            .to_srv_request();
        assert_eq!(async { flow.is_authenticated(&req).await }.await, true);
    }

    #[actix_web::test]
    async fn test_is_authenticate_request() {
        let flow = UsernameFlow {};
        let req = TestRequest::default()
            .uri("/login")
            .method(Method::POST)
            .to_srv_request();
        assert!(flow.is_authenticate_request(&req));
    }

    #[actix_web::test]
    async fn test_on_unauthenticated() {
        let flow = UsernameFlow {};
        let req = TestRequest::default().to_srv_request();
        let result = flow.on_unauthenticated(&req).await;
        assert!(result.is_ok());
    }
}
