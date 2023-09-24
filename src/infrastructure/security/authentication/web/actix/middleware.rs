use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;

use crate::infrastructure::security::authentication::web::actix::error::ErrUnauthorized;

pub struct AuthenticateStatusService<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthenticateStatusService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            if req.uri().path().ne("/login") && req.cookie("id").is_none() {
                return Ok(ServiceResponse::from_err(
                    ErrUnauthorized {},
                    req.request().to_owned(),
                ));
            }
            service.call(req).await
        })
    }
}

pub struct AuthenticateStateTransform {}

impl<S> Transform<S, ServiceRequest> for AuthenticateStateTransform
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Transform = AuthenticateStatusService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticateStatusService {
            service: Rc::new(service),
        }))
    }
}
