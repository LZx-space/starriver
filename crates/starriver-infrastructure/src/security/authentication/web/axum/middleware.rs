use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use futures_util::future::BoxFuture;
use std::marker::PhantomData;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

use crate::error::error::{ApiError, Cause};
use crate::security::authentication::core::credential::Credential;
use crate::security::authentication::core::principal::Principal;

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
    S::Error: Into<ApiError>,
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
    S::Error: Into<ApiError>,
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
    type Error = ApiError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(|e| e.into())
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut service = self.service.clone();
        let authenticator = self.authenticator.clone();
        let authentication_flow = self.authentication_flow.clone();
        Box::pin(async move {
            if authentication_flow.is_authenticate_request(&req).await {
                let ctx = authentication_flow
                    .extract_credential(req)
                    .await
                    .map_err(|e| ApiError::new(Cause::ClientBadRequest, e.to_string()))?;
                return match authentication_flow.authenticate(&authenticator, &ctx).await {
                    Ok(principal) => Ok(authentication_flow
                        .on_authenticate_success(&ctx, principal)
                        .await),
                    Err(err) => Ok(authentication_flow.on_authenticate_failure(&ctx, err).await),
                };
            }

            if authentication_flow
                .is_access_require_authentication(&req)
                .await
                && !authentication_flow.is_authenticated(&req).await
            {
                return Ok(authentication_flow.on_unauthenticated(req).await);
            }
            service.call(req).await.map_err(|e| e.into())
        })
    }
}
