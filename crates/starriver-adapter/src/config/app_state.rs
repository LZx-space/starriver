use axum::extract::FromRef;
use sea_orm::{Database, DatabaseConnection};
use starriver_application::article_service::ArticleApplication;
use starriver_application::category_service::CategoryApplication;
use starriver_application::user_service::UserApplication;
use starriver_infrastructure::security::authentication::web::config::AuthConfig;
use starriver_infrastructure::service::cache_service::VerificationCodeCache;
use starriver_infrastructure::service::config_service::{AppConfig, Assets};
use starriver_infrastructure::service::email_service::EmailClient;
use starriver_infrastructure::util::regex_patterns::Patterns;
use std::sync::Arc;

/// 应用的各个状态
#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub assets: Assets,
    pub auth_cfg: AuthConfig,
    pub patterns: Patterns,
    pub email_client: EmailClient,
    pub verification_code_cache: VerificationCodeCache,
    //////////////////////////////////////////
    pub user_application: Arc<UserApplication>,
    pub article_application: Arc<ArticleApplication>,
    pub category_application: Arc<CategoryApplication>,
}

impl AppState {
    pub async fn new(cfg: AppConfig) -> Result<Self, String> {
        let conn = Database::connect(cfg.database.url)
            .await
            .map_err(|e| e.to_string())?;
        let patterns = Patterns::new(cfg.regex);

        let email_client = EmailClient::new(cfg.email.smtp).map_err(|e| e.to_string())?;

        let verification_code_cache = VerificationCodeCache::new(cfg.email.verification_cache);

        let user_application = UserApplication::new(
            conn.clone(),
            email_client.clone(),
            verification_code_cache.clone(),
            patterns.clone(),
            cfg.aggregate.user.policy,
        )
        .into();
        let article_application = ArticleApplication::new(conn.clone(), cfg.assets.clone()).into();
        let category_application = CategoryApplication::new(conn.clone()).into();
        Ok(AppState {
            conn,
            assets: cfg.assets,
            auth_cfg: cfg.auth_cfg,
            patterns,
            email_client,
            verification_code_cache,
            user_application,
            article_application,
            category_application,
        })
    }
}

impl FromRef<AppState> for Patterns {
    fn from_ref(state: &AppState) -> Patterns {
        state.patterns.clone()
    }
}

impl FromRef<AppState> for AuthConfig {
    fn from_ref(state: &AppState) -> AuthConfig {
        state.auth_cfg.clone()
    }
}
