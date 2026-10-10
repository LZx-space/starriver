use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use futures_util::future::BoxFuture;
use starriver_shared_base::authentication::core::authentication_request::AuthenticationRequest;
use starriver_shared_base::authentication::core::authenticator::Authenticator;
use starriver_shared_base::authentication::core::principal::Principal;
use starriver_shared_base::authentication::web::authentication_request_extractor::AuthenticationRequestExtractor;
use starriver_shared_base::authentication::web::authentication_result_handler::{
    AuthenticationFailureHandler, AuthenticationSuccessHandler,
};
use starriver_shared_base::authentication::web::request_matcher::RequestMatcher;
use starriver_shared_base::authentication::web::timing_attack_protection::TimingAttackProtection;
use std::marker::PhantomData;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Instant;
use tower::{Layer, Service};

pub struct AuthenticationLayer<RM, CE, A, TAP, RS, RF, C, P> {
    login_request_matcher: Arc<RM>,
    credentials_extractor: Arc<CE>,
    authenticator: Arc<A>,
    timing_attack_protection: Arc<TAP>,
    success_handler: Arc<RS>,
    failure_handler: Arc<RF>,
    _c: PhantomData<C>,
    _p: PhantomData<P>,
}

impl<RM, CE, A, TAP, RS, RF, C, P> AuthenticationLayer<RM, CE, A, TAP, RS, RF, C, P>
where
    RM: RequestMatcher<Request = Request<Body>>,
    CE: AuthenticationRequestExtractor<Request = Request<Body>, AuthenticationRequest = C>,
    A: Authenticator<Request = C, Principal = P>,
    TAP: TimingAttackProtection,
    RS: AuthenticationSuccessHandler<Response = Response, Principal = P>,
    RF: AuthenticationFailureHandler<Response = Response>,
    C: AuthenticationRequest,
    P: Principal,
{
    pub fn new(
        login_request_matcher: RM,
        credentials_extractor: CE,
        authenticator: A,
        timing_attack_protection: TAP,
        success_handler: RS,
        failure_handler: RF,
    ) -> Self {
        AuthenticationLayer {
            login_request_matcher: Arc::new(login_request_matcher),
            credentials_extractor: Arc::new(credentials_extractor),
            authenticator: Arc::new(authenticator),
            timing_attack_protection: Arc::new(timing_attack_protection),
            success_handler: Arc::new(success_handler),
            failure_handler: Arc::new(failure_handler),
            _c: PhantomData,
            _p: PhantomData,
        }
    }
}

impl<S, RM, CE, A, TAP, RS, RF, C, P> Layer<S> for AuthenticationLayer<RM, CE, A, TAP, RS, RF, C, P>
where
    S: Service<Request<Body>, Response = Response>,
    RM: RequestMatcher<Request = Request<Body>>,
    CE: AuthenticationRequestExtractor<Request = Request<Body>, AuthenticationRequest = C>,
    A: Authenticator<Request = C, Principal = P>,
    TAP: TimingAttackProtection,
    RS: AuthenticationSuccessHandler<Response = Response, Principal = P>,
    RF: AuthenticationFailureHandler<Response = Response>,
    C: AuthenticationRequest,
    P: Principal,
{
    type Service = AuthenticationService<S, RM, CE, A, TAP, RS, RF, C, P>;

    fn layer(&self, service: S) -> Self::Service {
        AuthenticationService {
            service,
            login_request_matcher: self.login_request_matcher.clone(),
            credentials_extractor: self.credentials_extractor.clone(),
            authenticator: self.authenticator.clone(),
            timing_attack_protection: self.timing_attack_protection.clone(),
            success_handler: self.success_handler.clone(),
            failure_handler: self.failure_handler.clone(),
            _c: PhantomData,
            _p: PhantomData,
        }
    }
}

/// 当结构体有泛型时，#[derive(Clone)]会导致泛型也要满足Clone特性，这里则手动实现
impl<RM, CE, A, TAP, RS, RF, C, P> Clone for AuthenticationLayer<RM, CE, A, TAP, RS, RF, C, P> {
    fn clone(&self) -> Self {
        Self {
            login_request_matcher: self.login_request_matcher.clone(),
            credentials_extractor: self.credentials_extractor.clone(),
            authenticator: self.authenticator.clone(),
            timing_attack_protection: self.timing_attack_protection.clone(),
            success_handler: self.success_handler.clone(),
            failure_handler: self.failure_handler.clone(),
            _c: PhantomData,
            _p: PhantomData,
        }
    }
}

////////////////////////////////////////////////////////////////////////////

/// 认证服务，实现了tower的Service trait
pub struct AuthenticationService<S, RM, CE, A, TAP, RS, RF, C, P> {
    service: S,
    login_request_matcher: Arc<RM>,
    credentials_extractor: Arc<CE>,
    authenticator: Arc<A>,
    timing_attack_protection: Arc<TAP>,
    success_handler: Arc<RS>,
    failure_handler: Arc<RF>,
    _c: PhantomData<C>,
    _p: PhantomData<P>,
}

impl<S, RM, CE, A, TAP, RS, RF, C, P> Service<Request<Body>>
    for AuthenticationService<S, RM, CE, A, TAP, RS, RF, C, P>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    RM: RequestMatcher<Request = Request<Body>> + Send + Sync + 'static,
    CE: AuthenticationRequestExtractor<Request = Request<Body>, AuthenticationRequest = C>
        + Send
        + Sync
        + 'static,
    A: Authenticator<Request = C, Principal = P> + Send + Sync + 'static,
    TAP: TimingAttackProtection + Send + Sync + 'static,
    RS: AuthenticationSuccessHandler<Response = Response, Principal = P> + Send + Sync + 'static,
    RF: AuthenticationFailureHandler<Response = Response> + Send + Sync + 'static,
    C: AuthenticationRequest + 'static,
    P: Principal + 'static,
{
    type Response = Response;
    type Error = S::Error; // 业务内已处理所有Err，如果发生则需要HandleErrorLayer来使错误值到标准返回格式
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut service = self.service.clone();
        let request_matcher = self.login_request_matcher.clone();
        let credentials_extractor = self.credentials_extractor.clone();
        let authenticator = self.authenticator.clone();
        let timing_attack_protection = self.timing_attack_protection.clone();
        let success_handler = self.success_handler.clone();
        let failure_handler = self.failure_handler.clone();
        Box::pin(async move {
            if request_matcher.matches(&req).await {
                // 计时保护从解析开始，保证格式错误与正常认证失败的响应时长一致
                let start_at = Instant::now();
                let authenticated = async {
                    let credentials = credentials_extractor.extract(req).await?;
                    authenticator.authenticate(&credentials).await
                }
                .await;
                timing_attack_protection
                    .fixed_duration_delay(start_at)
                    .await;
                return match authenticated {
                    Ok(result) => Ok(success_handler.on_authentication_success(result).await),
                    Err(err) => Ok(failure_handler.on_authentication_failure(err).await),
                };
            }
            service.call(req).await
        })
    }
}

////////////////////////////////////////////////////////////////////////////

/// 当结构体有泛型时，#[derive(Clone)]会导致泛型也要满足Clone特性，这里则手动实现
impl<S: Clone, RM, CE, A, TAP, RS, RF, C, P> Clone
    for AuthenticationService<S, RM, CE, A, TAP, RS, RF, C, P>
{
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
            login_request_matcher: self.login_request_matcher.clone(),
            credentials_extractor: self.credentials_extractor.clone(),
            authenticator: self.authenticator.clone(),
            timing_attack_protection: self.timing_attack_protection.clone(),
            success_handler: self.success_handler.clone(),
            failure_handler: self.failure_handler.clone(),
            _c: PhantomData,
            _p: PhantomData,
        }
    }
}
