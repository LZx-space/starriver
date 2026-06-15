use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use starriver_identity_application::{
    dto::user_dto::req::UserValidateCxt, use_case::user_interactor::UserApplicationService,
};
use starriver_identity_domain::{
    authentication_service::AuthenticationDomainService,
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
            query::user_query::DefaultUserQuery,
            repository::{
                security_event_repository::DefaultSecurityEventRepository,
                user_repository::DefaultUserRepository,
            },
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
    pub user_service: Arc<
        UserApplicationService<
            DefaultConnection,
            DefaultUserQuery,
            DefaultUserRepository,
            DefaultSecurityEventRepository,
            SmtpVerificationService,
            Argon2PasswordEncoder,
        >,
    >,
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
            max_attempts: cfg.bad_password.max_attempts as usize,
        };

        let auth_service = AuthenticationDomainService::new(bad_password_policy, password_encoder);
        let user_service = UserApplicationService::new(
            DefaultConnection::new(conn),
            DefaultUserQuery,
            DefaultUserRepository,
            DefaultSecurityEventRepository,
            verification_code_service,
            user_factory,
            auth_service,
        )
        .into();
        Ok(IdentityState {
            auth,
            email_spec,
            username_spec,
            password_spec,
            user_service,
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
