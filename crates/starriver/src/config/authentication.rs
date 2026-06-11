use std::sync::Arc;

use starriver_identity_adapter::{
    UserApplicationService,
    port_out::{
        persistence::{
            query::user_query_port::DefaultUserQueryPort,
            repository::{
                security_event_repository::DefaultSecurityEventRepository,
                user_repository::DefaultUserRepository,
            },
        },
        service::{
            email_verification_port::SmtpVerificationPort, password_encoder::Argon2PasswordEncoder,
        },
    },
};
use starriver_shared_base::{
    authentication::{PrincipalClaims, UsernamePasswordCredentials},
    middleware::authentication::core::{authenticator::Authenticator, error::AuthenticationError},
};
use starriver_shared_framework::{
    config::Auth,
    middleware::authentication::{
        default_impl::{
            AuthenticatedUser, DefaultAuthenticationFailureHandler,
            DefaultAuthenticationSuccessHandler, DefaultCredentialsExtractor, LoginRequestMatcher,
            TokioTimingAttackProtection,
        },
        middleware::AuthenticationLayer,
    },
};
use time::Duration;

pub struct UsernamePasswordAuthenticator {
    pub user_service: Arc<
        UserApplicationService<
            DefaultUserQueryPort,
            DefaultUserRepository,
            DefaultSecurityEventRepository,
            SmtpVerificationPort,
            Argon2PasswordEncoder,
        >,
    >,
    pub cfg: Arc<Auth>,
}

impl Authenticator for UsernamePasswordAuthenticator {
    type Credentials = UsernamePasswordCredentials;
    type Principal = AuthenticatedUser;

    async fn authenticate(
        &self,
        credentials: &Self::Credentials,
    ) -> Result<Self::Principal, AuthenticationError> {
        let detail = self.user_service.authenticate(credentials).await?;
        let claims = PrincipalClaims::new(
            Duration::hours(self.cfg.jws_exp_hours as i64),
            detail.id,
            detail.username,
            detail.email,
        );
        Ok(AuthenticatedUser(claims))
    }
}

/////////////////////////////////////////////////////////////////

pub fn build_authentication_layer<A>(
    authenticator: A,
    cfg: Arc<Auth>,
) -> AuthenticationLayer<
    LoginRequestMatcher,
    DefaultCredentialsExtractor,
    A,
    TokioTimingAttackProtection,
    DefaultAuthenticationSuccessHandler,
    DefaultAuthenticationFailureHandler,
    UsernamePasswordCredentials,
    AuthenticatedUser,
>
where
    A: Authenticator<Credentials = UsernamePasswordCredentials, Principal = AuthenticatedUser>,
{
    AuthenticationLayer::new(
        LoginRequestMatcher::default(),
        DefaultCredentialsExtractor {},
        authenticator,
        TokioTimingAttackProtection::default(),
        DefaultAuthenticationSuccessHandler::new(cfg),
        DefaultAuthenticationFailureHandler {},
    )
}
