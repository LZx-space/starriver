use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use starriver_identity_application::service::user_service::UserApplicationService;
use starriver_identity_domain::user::policy::BadPasswordPolicy;
use starriver_shared_base::regex_patterns::Patterns;
use starriver_shared_framework::principal::Auth;

use crate::{
    config::IdentityConfig,
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

/// 应用的各个状态
#[derive(Clone)]
pub struct IdentityState {
    pub conn: DatabaseConnection,
    pub patterns: Patterns,
    pub auth: Auth,
    //////////////////////////////////////////
    pub user_service: Arc<
        UserApplicationService<
            DefaultUserQueryPort,
            DefaultUserRepository<DatabaseConnection>,
            SmtpVerificationPort,
            Argon2PasswordEncoder,
        >,
    >,
}

impl IdentityState {
    pub async fn new(
        conn: DatabaseConnection,
        auth: Auth,
        cfg: &IdentityConfig,
    ) -> Result<Self, String> {
        let patterns = Patterns::new(
            &cfg.regexes.email,
            &cfg.regexes.username,
            &cfg.regexes.password,
        )
        .map_err(|e| e.to_string())?;
        ////////////////////////////////////////////////////////

        let query = DefaultUserQueryPort { conn: conn.clone() };
        let repo = DefaultUserRepository::new(conn.clone(), patterns.clone());
        let verification_code_port =
            SmtpVerificationPort::new(&cfg.email_smtp).map_err(|e| e.to_string())?;
        let bad_password_policy = BadPasswordPolicy {
            window_minutes: cfg.bad_password.window_minutes,
            max_attempts: cfg.bad_password.max_attempts as usize,
        };
        let password_encoder = Argon2PasswordEncoder::default().into();
        let user_service = UserApplicationService::new(
            query,
            repo,
            verification_code_port,
            patterns.clone(),
            bad_password_policy,
            password_encoder,
        )
        .into();
        Ok(IdentityState {
            conn,
            patterns,
            auth,
            user_service,
        })
    }
}

impl FromRef<IdentityState> for Patterns {
    fn from_ref(state: &IdentityState) -> Patterns {
        state.patterns.clone()
    }
}

impl FromRef<IdentityState> for Auth {
    fn from_ref(state: &IdentityState) -> Auth {
        state.auth.clone()
    }
}
