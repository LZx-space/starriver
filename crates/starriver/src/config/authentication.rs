use std::sync::Arc;

use starriver_identity_adapter::{
    AuthenticationInteractor,
    port_out::{
        persistence::{
            security_event_port::DefaultSecurityEventPort, user_repository::DefaultUserRepository,
        },
        service::password_encoder::Argon2PasswordEncoder,
    },
};
use starriver_shared_base::authentication::{
    authentication_request::IdentifierPasswordRequest,
    core::{
        authentication_result::AuthenticationResult, authenticator::Authenticator,
        error::AuthenticationError,
    },
    principal::DefaultUser,
};
use starriver_shared_infra::{
    config::Auth,
    db::DefaultConnection,
    middleware::authentication::{
        default_impl::{
            DefaultAuthenticationFailureHandler, DefaultAuthenticationRequestExtractor,
            DefaultAuthenticationSuccessHandler, LoginRequestMatcher, TokioTimingAttackProtection,
        },
        middleware::AuthenticationLayer,
    },
};

pub struct IdentifierPasswordAuthenticator {
    pub auth_service: Arc<
        AuthenticationInteractor<
            DefaultConnection,
            DefaultUserRepository,
            DefaultSecurityEventPort,
            Argon2PasswordEncoder,
        >,
    >,
}

impl Authenticator for IdentifierPasswordAuthenticator {
    type Request = IdentifierPasswordRequest;
    type Principal = DefaultUser;

    async fn authenticate(
        &self,
        req: &Self::Request,
    ) -> Result<AuthenticationResult<Self::Principal>, AuthenticationError> {
        let detail = self.auth_service.authenticate(req).await?;
        let user = DefaultUser::new(detail.id, detail.username, detail.email);
        Ok(AuthenticationResult::new(user))
    }
}

/////////////////////////////////////////////////////////////////

pub fn build_authentication_layer<A>(
    authenticator: A,
    cfg: Arc<Auth>,
) -> AuthenticationLayer<
    LoginRequestMatcher,
    DefaultAuthenticationRequestExtractor,
    A,
    TokioTimingAttackProtection,
    DefaultAuthenticationSuccessHandler,
    DefaultAuthenticationFailureHandler,
    IdentifierPasswordRequest,
    DefaultUser,
>
where
    A: Authenticator<Request = IdentifierPasswordRequest, Principal = DefaultUser>,
{
    AuthenticationLayer::new(
        LoginRequestMatcher::default(),
        DefaultAuthenticationRequestExtractor {},
        authenticator,
        TokioTimingAttackProtection::default(),
        DefaultAuthenticationSuccessHandler::new(cfg),
        DefaultAuthenticationFailureHandler {},
    )
}
