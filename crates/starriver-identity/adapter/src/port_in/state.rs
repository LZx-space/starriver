use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use starriver_identity_application::{
    dto::user_dto::req::UserValidateCxt,
    use_case::{
        authentication_interactor::AuthenticationInteractor,
        security_event_interactor::SecurityEventInteractor, user_interactor::UserInteractor,
    },
};
use starriver_identity_domain::{
    password_service::PasswordDomainService,
    user::{
        factory::UserFactory,
        policy::BadPasswordPolicy,
        specification::{EmailSpec, PasswordSpec, UsernameSpec},
    },
};
use starriver_shared_base::regex_patterns::Patterns;
use starriver_shared_framework::{config::Auth, db::DefaultConnection};

use crate::{
    config::IdentityConfig,
    port_out::{
        persistence::{
            security_event_port::DefaultSecurityEventPort, user_query::DefaultUserQuery,
            user_repository::DefaultUserRepository,
        },
        service::{
            email_verification_service::SmtpVerificationService,
            password_encoder::Argon2PasswordEncoder,
        },
    },
};

/// 应用的各个状态
#[derive(Clone)]
pub struct IdentityState {
    pub auth: Arc<Auth>,
    pub email_spec: Arc<EmailSpec>,
    pub username_spec: Arc<UsernameSpec>,
    pub password_spec: Arc<PasswordSpec>,
    //////////////////////////////////////////
    pub user_interactor: Arc<
        UserInteractor<
            DefaultConnection,
            DefaultUserQuery,
            DefaultUserRepository,
            SmtpVerificationService,
            Argon2PasswordEncoder,
        >,
    >,
    pub authentication_interactor: Arc<
        AuthenticationInteractor<
            DefaultConnection,
            DefaultUserRepository,
            DefaultSecurityEventPort,
            Argon2PasswordEncoder,
        >,
    >,
    pub security_event_interactor:
        Arc<SecurityEventInteractor<DefaultConnection, DefaultSecurityEventPort>>,
}

impl IdentityState {
    pub async fn new(
        conn: DatabaseConnection,
        auth: Arc<Auth>,
        cfg: &IdentityConfig,
    ) -> Result<Self, String> {
        let patterns = Patterns::new(
            &cfg.regexes.email,
            &cfg.regexes.username,
            &cfg.regexes.password,
        )
        .map_err(|e| e.to_string())?;
        let email_spec: Arc<_> = EmailSpec::new(patterns.email).into();
        let username_spec: Arc<_> = UsernameSpec::new(patterns.username).into();
        let password_spec: Arc<_> = PasswordSpec::new(patterns.password).into();

        let password_encoder: Arc<Argon2PasswordEncoder> = Argon2PasswordEncoder::default().into();
        ////////////////////////////////////////////////////////

        let verification_code_service =
            SmtpVerificationService::new(&cfg.email_smtp).map_err(|e| e.to_string())?;

        let user_factory = UserFactory::new(
            email_spec.clone(),
            username_spec.clone(),
            password_spec.clone(),
            password_encoder.clone(),
        );
        let bad_password_policy = BadPasswordPolicy {
            window_minutes: cfg.bad_password.window_minutes,
            max_attempts: cfg.bad_password.max_attempts,
            lockout_minutes: cfg.bad_password.lockout_minutes,
        };

        let pwd_service = PasswordDomainService::new(
            bad_password_policy,
            password_encoder,
            password_spec.clone(),
        );

        let conn = DefaultConnection::new(conn.clone());
        let user_interactor = UserInteractor::new(
            conn.clone(),
            DefaultUserQuery,
            DefaultUserRepository,
            user_factory,
            verification_code_service,
            pwd_service.clone(),
        )
        .into();

        let authentication_interactor = AuthenticationInteractor::new(
            conn.clone(),
            DefaultUserRepository,
            DefaultSecurityEventPort,
            pwd_service.clone(),
        )
        .into();

        let security_event_interactor =
            SecurityEventInteractor::new(conn.clone(), DefaultSecurityEventPort).into();

        Ok(IdentityState {
            auth,
            email_spec,
            username_spec,
            password_spec,
            user_interactor,
            authentication_interactor,
            security_event_interactor,
        })
    }
}

impl FromRef<IdentityState> for UserValidateCxt {
    fn from_ref(state: &IdentityState) -> UserValidateCxt {
        UserValidateCxt {
            username_spec: state.username_spec.clone(),
            password_spec: state.password_spec.clone(),
        }
    }
}

impl FromRef<IdentityState> for Arc<Auth> {
    fn from_ref(state: &IdentityState) -> Arc<Auth> {
        state.auth.clone()
    }
}
