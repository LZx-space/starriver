use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

use crate::security::authentication::core::credential::Credential;
use crate::security::authentication::core::principal::Principal;

use crate::model::err::CodedErr;
use crate::security::authentication::core::authenticator::Authenticator;
use crate::security::authentication::web::flow::AuthenticationFlow;

#[derive(Clone)]
pub struct AuthenticationLayer<A, F, C, P> {
    pub authenticator: Arc<A>,
    pub authentication_flow: Arc<F>,
    _c: PhantomData<C>,
    _p: PhantomData<P>,
}

impl<A, F, C, P> AuthenticationLayer<A, F, C, P>
where
    A: Authenticator<Credential = C, Principal = P>,
    F: AuthenticationFlow<
            Request = Request<Body>,
            Response = Response,
            Credential = C,
            Principal = P,
            Authenticator = A,
        >,
    C: Credential,
    P: Principal,
{
    pub fn new(authenticator: A, authentication_flow: F) -> Self {
        AuthenticationLayer {
            authenticator: Arc::new(authenticator),
            authentication_flow: Arc::new(authentication_flow),
            _c: PhantomData::<C>::default(),
            _p: PhantomData::<P>::default(),
        }
    }
}

impl<S, A, F, C, P> Layer<S> for AuthenticationLayer<A, F, C, P>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + Sync + 'static,
    S::Error: Into<CodedErr>,
    A: Authenticator<Credential = C, Principal = P>,
    F: AuthenticationFlow<
            Request = Request<Body>,
            Response = Response,
            Credential = C,
            Principal = P,
            Authenticator = A,
        >,
    C: Credential,
    P: Principal,
{
    type Service = AuthenticationService<S, A, F, C, P>;

    fn layer(&self, service: S) -> Self::Service {
        AuthenticationService {
            service,
            authenticator: self.authenticator.clone(),
            authentication_flow: self.authentication_flow.clone(),
            _c: PhantomData::<C>::default(),
            _p: PhantomData::<P>::default(),
        }
    }
}

/// 认证服务，实现了tower的Service trait
#[derive(Clone)]
pub struct AuthenticationService<S, A, F, C, P> {
    service: S,
    authenticator: Arc<A>,
    authentication_flow: Arc<F>,
    _c: PhantomData<C>,
    _p: PhantomData<P>,
}

impl<S, A, F, C, P> Service<Request<Body>> for AuthenticationService<S, A, F, C, P>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + Sync + 'static,
    S::Error: Into<CodedErr>,
    S::Future: Send + 'static,
    A: Authenticator<Credential = C, Principal = P> + Send + Sync + 'static,
    F: AuthenticationFlow<
            Request = Request<Body>,
            Response = Response,
            Credential = C,
            Principal = P,
            Authenticator = A,
        > + Send
        + Sync
        + 'static,
    C: Credential + Send + Sync + 'static,
    P: Principal,
{
    type Response = Response;
    type Error = CodedErr;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(|e| e.into())
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let mut service = self.service.clone();
        let authenticator = self.authenticator.clone();
        let authentication_flow = self.authentication_flow.clone();
        Box::pin(async move {
            if authentication_flow.is_authenticate_request(&req).await {
                let credential = authentication_flow
                    .extract_credential(&mut req)
                    .await
                    .map_err(|e| CodedErr::new("1000".to_string(), e.to_string()))?;
                return match authentication_flow
                    .authenticate(&authenticator, &credential)
                    .await
                {
                    Ok(principal) => authentication_flow
                        .on_authenticate_success(&req, principal)
                        .await
                        .map_err(|e| CodedErr::new("1000".to_string(), e.to_string())),
                    Err(err) => authentication_flow
                        .on_authenticate_failure(&req, err)
                        .await
                        .map_err(|e| CodedErr::new("1000".to_string(), e.to_string())),
                };
            }

            if authentication_flow
                .is_access_require_authentication(&req)
                .await
                && !authentication_flow.is_authenticated(&req).await
            {
                return authentication_flow
                    .on_unauthenticated(&req)
                    .await
                    .map_err(|e| CodedErr::new("1000".to_string(), e.to_string()));
            }
            service.call(req).await.map_err(|e| e.into())
        })
    }
}
