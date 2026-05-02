use axum::extract::FromRef;
use sea_orm::{Database, DatabaseConnection};
use starriver_application::article_service::ArticleApplication;
use starriver_application::category_service::CategoryApplication;
use starriver_application::user_service::UserApplication;
use starriver_base::security::authentication::web::config::AuthConfig;
use starriver_base::service::cache_service::VerificationCodeCache;
use starriver_base::service::config_service::{AppConfig, Uploads};
use starriver_base::service::email_service::EmailClient;
use starriver_base::util::regex_patterns::Patterns;
use std::sync::Arc;

/// 应用的各个状态
#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub auth_cfg: AuthConfig,
    pub upload_cfg: Uploads,
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
        let article_application = ArticleApplication::new(conn.clone(), cfg.uploads.clone()).into();
        let category_application = CategoryApplication::new(conn.clone()).into();
        Ok(AppState {
            conn,
            auth_cfg: cfg.auth,
            upload_cfg: cfg.uploads,
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
