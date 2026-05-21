use std::sync::Arc;

use sea_orm::ConnectionTrait;
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

pub struct UsernamePasswordAuthenticator<T> {
    pub user_service: Arc<
        UserApplicationService<
            DefaultUserQueryPort,
            DefaultUserRepository<T>,
            DefaultSecurityEventRepository,
            SmtpVerificationPort,
            Argon2PasswordEncoder,
        >,
    >,
}

impl<T> Authenticator for UsernamePasswordAuthenticator<T>
where
    T: ConnectionTrait + Send,
{
    type Credentials = UsernamePasswordCredentials;
    type Principal = AuthenticatedUser;

    async fn authenticate(
        &self,
        credentials: &Self::Credentials,
    ) -> Result<Self::Principal, AuthenticationError> {
        let detail = self.user_service.authenticate(credentials).await?;
        let claims = PrincipalClaims::new(detail.id, detail.username, detail.email);
        Ok(AuthenticatedUser(claims))
    }
}

/////////////////////////////////////////////////////////////////

pub fn build_authentication_layer<A>(
    authenticator: A,
    cfg: Auth,
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
