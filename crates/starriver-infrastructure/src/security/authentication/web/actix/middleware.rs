use crate::model::err::CodedErr;
use crate::security::authentication::core::authenticator::Authenticator;
use crate::security::authentication::web::flow::AuthenticationFlow;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::marker::PhantomData;
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct AuthenticationTransform<A, F, C, P> {
    pub authenticator: Arc<A>,
    pub authentication_flow: Arc<F>,
    _c: PhantomData<C>,
    _p: PhantomData<P>,
}

impl<A, F, C, P> AuthenticationTransform<A, F, C, P> {
    pub fn new(authenticator: A, authentication_flow: F) -> Self {
        AuthenticationTransform {
            authenticator: Arc::new(authenticator),
            authentication_flow: Arc::new(authentication_flow),
            _c: Default::default(),
            _p: Default::default(),
        }
    }
}

impl<S, A, F, C, P> Transform<S, ServiceRequest> for AuthenticationTransform<A, F, C, P>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    A: Authenticator<Credential = C, Principal = P> + Send + Sync + 'static,
    F: AuthenticationFlow<
            Request = ServiceRequest,
            Response = ServiceResponse,
            Credential = C,
            Principal = P,
            Authenticator = A,
        > + Send
        + Sync
        + 'static,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Transform = AuthenticationService<S, A, F, C, P>; // 这个Transform的类型是S，不是本身，更好的命名是Service
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationService {
            service: Arc::new(service),
            authenticator: self.authenticator.clone(),
            authentication_flow: self.authentication_flow.clone(),
            _c: PhantomData::<C>::default(),
            _p: PhantomData::<P>::default(),
        }))
    }
}

pub struct AuthenticationService<S, A, F, C, P> {
    service: Arc<S>,
    authenticator: Arc<A>,
    authentication_flow: Arc<F>,
    _c: PhantomData<C>,
    _p: PhantomData<P>,
}

impl<S, A, F, C, P> Service<ServiceRequest> for AuthenticationService<S, A, F, C, P>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    A: Authenticator + Send + Sync + 'static,
    F: AuthenticationFlow<
            Request = ServiceRequest,
            Response = ServiceResponse,
            Credential = C,
            Principal = P,
            Authenticator = A,
        > + Send
        + Sync
        + 'static,
{
    type Response = ServiceResponse;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Arc::clone(&self.service);
        let authenticator = Arc::clone(&self.authenticator);
        let authentication_flow = Arc::clone(&self.authentication_flow);
        Box::pin(async move {
            if authentication_flow.is_authenticate_request(&req) {
                let credential = authentication_flow
                    .extract_credential(&mut req)
                    .await
                    .map_err(|e| Error::from(CodedErr::new("1000".to_string(), e.to_string())))?;
                return match authentication_flow
                    .authenticate(authenticator.as_ref(), &credential)
                    .await
                {
                    Ok(principal) => authentication_flow
                        .on_authenticate_success(&req, principal)
                        .await
                        .map_err(|e| Error::from(CodedErr::new("1000".to_string(), e.to_string()))),
                    Err(err) => authentication_flow
                        .on_authenticate_failure(&req, err)
                        .await
                        .map_err(|e| Error::from(CodedErr::new("1000".to_string(), e.to_string()))),
                };
            }
            if authentication_flow.is_access_require_authentication(&req)
                && !authentication_flow.is_authenticated(&req).await
            {
                return authentication_flow
                    .on_unauthenticated(&req)
                    .await
                    .map_err(|e| Error::from(CodedErr::new("1000".to_string(), e.to_string())));
            }
            service.call(req).await
        })
    }
}
