use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use futures_util::future::BoxFuture;
use std::marker::PhantomData;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

use crate::security::authentication::core::credential::Credential;
use crate::security::authentication::core::principal::Principal;

use crate::security::authentication::core::authenticator::Authenticator;
use crate::security::authentication::web::flow::AuthenticationFlow;

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

/// 当结构体有泛型时，#[derive(Clone)]会导致泛型也要满足Clone特性，这里则手动实现
impl<A, F, C, P> Clone for AuthenticationLayer<A, F, C, P> {
    fn clone(&self) -> Self {
        Self {
            authenticator: self.authenticator.clone(),
            authentication_flow: self.authentication_flow.clone(),
            _c: self._c.clone(),
            _p: self._p.clone(),
        }
    }
}

// ---------------------------------------------------------------------------------

/// 认证服务，实现了tower的Service trait
pub struct AuthenticationService<S: Clone, A, F, C, P> {
    service: S,
    authenticator: Arc<A>,
    authentication_flow: Arc<F>,
    _c: PhantomData<C>,
    _p: PhantomData<P>,
}

impl<S, A, F, C, P> Service<Request<Body>> for AuthenticationService<S, A, F, C, P>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + Sync + 'static,
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
    type Error = S::Error; // 业务内已处理所有Err，如果发生则需要HandleErrorLayer来使错误值到标准返回格式
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut service = self.service.clone();
        let authenticator = self.authenticator.clone();
        let authentication_flow = self.authentication_flow.clone();
        Box::pin(async move {
            // 如果是认证请求，则进行认证
            if authentication_flow.is_authenticate_request(&req).await {
                let credential = authentication_flow.extract_credential(req).await;
                let credential = match credential {
                    Ok(credential) => credential,
                    Err(e) => {
                        return Ok(authentication_flow.on_authenticate_failure(e).await);
                    }
                };
                return match authentication_flow
                    .authenticate(&authenticator, &credential)
                    .await
                {
                    Ok(principal) => {
                        Ok(authentication_flow.on_authenticate_success(principal).await)
                    }
                    Err(err) => Ok(authentication_flow.on_authenticate_failure(err).await),
                };
            }
            // 如果需要认证且未认证，则返回未认证响应
            if authentication_flow
                .is_access_require_authentication(&req)
                .await
                && !authentication_flow.is_authenticated(&req).await
            {
                return Ok(authentication_flow.on_unauthenticated(req).await);
            }
            service.call(req).await
        })
    }
}

/// 当结构体有泛型时，#[derive(Clone)]会导致泛型也要满足Clone特性，这里则手动实现
impl<S: Clone, A, F, C, P> Clone for AuthenticationService<S, A, F, C, P> {
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
            authenticator: self.authenticator.clone(),
            authentication_flow: self.authentication_flow.clone(),
            _c: self._c.clone(),
            _p: self._p.clone(),
        }
    }
}
