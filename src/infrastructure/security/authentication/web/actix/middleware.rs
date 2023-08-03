use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use std::future::{ready, Ready};
use std::task::{Context, Poll};

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
                // ready(service_response)
                todo!()
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
