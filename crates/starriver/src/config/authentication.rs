use std::sync::Arc;

use sea_orm::ConnectionTrait;
use starriver_identity_adapter::{
    UserApplicationService,
    port_out::{
        persistence::{
            query::user_query_port::DefaultUserQueryPort,
            repository::user_repository::DefaultUserRepository,
        },
        service::{
            email_verification_port::SmtpVerificationPort, password_encoder::Argon2PasswordEncoder,
        },
    },
};
use starriver_shared_base::authentication::{PrincipalClaims, UsernamePasswordCredentials};
use starriver_shared_framework::principal::{Auth, AuthenticatedUser};

use starriver_shared_framework::middleware::authentication::{
    _default_impl::{
        DefaultAuthenticationFailureHandler, DefaultAuthenticationSuccessHandler,
        DefaultCredentialsExtractor, LoginRequestMatcher,
    },
    core::authenticator::{AuthenticationError, Authenticator},
    web::{middleware::AuthenticationLayer, timing_attack_protection::TokioTimingAttackProtection},
};

pub struct UsernamePasswordAuthenticator<T> {
    pub user_service: Arc<
        UserApplicationService<
            DefaultUserQueryPort,
            DefaultUserRepository<T>,
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
        self.user_service
            .authenticate(credentials)
            .await
            .map(|e| {
                let claims = PrincipalClaims::new(e.id, e.username, e.email);
                AuthenticatedUser(claims)
            })
            .map_err(|_| AuthenticationError::InnerError)
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
