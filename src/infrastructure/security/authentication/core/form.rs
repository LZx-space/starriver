use actix_web::body::MessageBody;
use std::fmt::Debug;
use std::future::{ready, Ready};
use std::task::{Context, Poll};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use crate::infrastructure::security::authentication::core::credentials::Credentials;
use crate::infrastructure::security::authentication::core::credentials_repository::CredentialsRepository;
use crate::infrastructure::security::authentication::core::principal::Principal;

/// 用户名密码类型的凭证
#[derive(Debug, Serialize, Deserialize)]
pub struct UsernamePasswordCredentials {
    username: String,
    password: String,
}

impl UsernamePasswordCredentials {
    pub fn new(username: String, password: String) -> Self {
        UsernamePasswordCredentials { username, password }
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

impl Credentials for UsernamePasswordCredentials {}

pub struct UserCredentialsRepository {}

impl CredentialsRepository<String, UsernamePasswordCredentials> for UserCredentialsRepository {
    fn find_by_id(&self, credentials_id: &String) -> Option<UsernamePasswordCredentials> {
        let credentials = UsernamePasswordCredentials {
            username: credentials_id.clone(),
            password: "password".to_string(),
        };
        Some(credentials)
    }
}

pub struct UsernamePasswordCredentialsAuthenticator {
    credentials_repository: Box<dyn CredentialsRepository<String, UsernamePasswordCredentials>>,
}

impl UsernamePasswordCredentialsAuthenticator {
    pub fn new(
        repo: Box<dyn CredentialsRepository<String, UsernamePasswordCredentials>>,
    ) -> UsernamePasswordCredentialsAuthenticator {
        UsernamePasswordCredentialsAuthenticator {
            credentials_repository: repo,
        }
    }
}

impl Authenticator<UsernamePasswordCredentials> for UsernamePasswordCredentialsAuthenticator {
    fn authenticate(
        &self,
        principal: &mut Principal<UsernamePasswordCredentials>,
    ) -> Result<(), AuthenticationError> {
        let credentials = principal.credentials();
        let credentials_in_repo = self
            .credentials_repository
            .find_by_id(&credentials.username);
        match credentials_in_repo {
            None => Err(AuthenticationError::UsernameNotFound),
            Some(credentials_in_repo) => {
                if credentials_in_repo.password == credentials.password {
                    principal.set_authenticated();
                    Ok(())
                } else {
                    Err(AuthenticationError::BadPassword)
                }
            }
        }
    }
}

pub struct AuthenticateStatusService<S> {
    service: S,
}

impl<S: Service<ServiceRequest>> Service<ServiceRequest> for AuthenticateStatusService<S> {
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match req.cookie("id") {
            None => {
                println!("--> no session cookie");
                let response = HttpResponse::new(StatusCode::UNAUTHORIZED);

                let service_response = ServiceResponse::new(req.request().clone(), response);
                ready(Ok(service_response))
            }
            Some(val) => {
                println!("--> with session cookie {:#?}", val);
                self.service.call(req)
            }
        }
    }
}

pub struct AuthenticateStateTransform {}

impl<S: Service<ServiceRequest>> Transform<S, ServiceRequest> for AuthenticateStateTransform {
    type Response = S::Response;
    type Error = S::Error;
    type Transform = AuthenticateStatusService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticateStatusService { service }))
    }
}
