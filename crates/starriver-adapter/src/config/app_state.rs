use sea_orm::DatabaseConnection;
use starriver_application::service::blog_service::BlogApplication;
use starriver_application::service::user_service::UserApplication;
use starriver_infrastructure::service::dictionary::dictionary_service::Dictionary;
use std::sync::Arc;

/// 应用的各个状态
#[derive(Clone)]
pub struct AppState {
    pub conn: &'static DatabaseConnection,
    pub user_application: Arc<UserApplication>,
    pub blog_application: Arc<BlogApplication>,
    pub dictionary: Arc<Dictionary>,
}

impl AppState {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        let user_application = Arc::new(UserApplication::new(conn));
        let blog_application = Arc::new(BlogApplication::new(conn));
        let dictionary = Arc::new(Dictionary::new(conn));
        AppState {
            conn,
            user_application,
            blog_application,
            dictionary,
        }
    }
}
