use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use starriver_identity_application::{
    common::regex_patterns::Patterns, service::user_service::UserApplicationService,
};
use starriver_identity_domain::aggregate::user_policy::BadPassswordPolicy;

use crate::{
    config::{AuthConfig, IdentityConfig},
    port_out::{
        persistence::{
            query::user_query_port::DefaultUserQueryPort,
            repository::user_repository::DefaultUserRepository,
        },
        service::{
            password_encoder::DefaultPasswordEncoder,
            verification_code_port::DefaultVerificationCodePort,
        },
    },
};

/// 应用的各个状态
#[derive(Clone)]
pub struct IdentityState {
    pub conn: DatabaseConnection,
    pub patterns: Patterns,
    pub auth_cfg: AuthConfig,
    //////////////////////////////////////////
    pub user_service: Arc<
        UserApplicationService<
            DefaultUserQueryPort,
            DefaultUserRepository<DatabaseConnection>,
            DefaultVerificationCodePort,
            DefaultPasswordEncoder,
        >,
    >,
}

impl IdentityState {
    pub async fn new(conn: DatabaseConnection, cfg: &IdentityConfig) -> Result<Self, String> {
        let patterns = Patterns::new(
            &cfg.regexes.email,
            &cfg.regexes.username,
            &cfg.regexes.password,
        )
        .map_err(|e| e.to_string())?;
        let auth_cfg = cfg.auth_cfg.clone();
        ////////////////////////////////////////////////////////

        let query = DefaultUserQueryPort { conn: conn.clone() };
        let repo = DefaultUserRepository::new(conn.clone(), patterns.clone());
        let verification_code_port = DefaultVerificationCodePort {};
        let bad_password_policy = BadPassswordPolicy {
            window_minutes: cfg.bad_password.window_minutes,
            max_attempts: cfg.bad_password.max_attempts as usize,
        };
        let password_encoder = DefaultPasswordEncoder {};
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
            auth_cfg,
            user_service,
        })
    }
}

impl FromRef<IdentityState> for Patterns {
    fn from_ref(state: &IdentityState) -> Patterns {
        state.patterns.clone()
    }
}

impl FromRef<IdentityState> for AuthConfig {
    fn from_ref(state: &IdentityState) -> AuthConfig {
        state.auth_cfg.clone()
    }
}
