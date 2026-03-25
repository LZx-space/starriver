use sea_orm::{Database, DatabaseConnection};
use starriver_application::blog_service::BlogApplication;
use starriver_application::user_service::UserApplication;
use starriver_infrastructure::service::cache_service::VerificationCodeCache;
use starriver_infrastructure::service::config_service::AppConfig;
use starriver_infrastructure::service::dictionary::dictionary_service::Dictionary;
use starriver_infrastructure::service::email_service::EmailClient;
use starriver_infrastructure::util::regex_patterns::Patterns;
use std::sync::Arc;

/// 应用的各个状态
#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub patterns: Patterns,
    pub email_client: Arc<EmailClient>,
    pub verification_code_cache: Arc<VerificationCodeCache>,
    pub user_application: Arc<UserApplication>,
    pub blog_application: Arc<BlogApplication>,
    pub dictionary: Arc<Dictionary>,
}

impl AppState {
    pub async fn new(cfg: AppConfig) -> Result<Self, String> {
        let conn = Database::connect(cfg.database.url)
            .await
            .expect("create a DatabaseConnection failed");
        let patterns = Patterns::new(cfg.regex);

        let email_client = EmailClient::new(cfg.email).map_err(|e| e.to_string())?;
        let email_client = Arc::new(email_client);

        let verification_code_cache = VerificationCodeCache::new(cfg.email_verification_cache);
        let verification_code_cache = Arc::new(verification_code_cache);

        let user_application = Arc::new(UserApplication::new(
            conn.clone(),
            email_client.clone(),
            verification_code_cache.clone(),
            patterns.clone(),
        ));
        let blog_application = Arc::new(BlogApplication::new(conn.clone()));
        let dictionary = Arc::new(Dictionary::new(conn.clone()));
        Ok(AppState {
            conn,
            patterns,
            email_client,
            verification_code_cache,
            user_application,
            blog_application,
            dictionary,
        })
    }
}
