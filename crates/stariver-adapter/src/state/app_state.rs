use sea_orm::DatabaseConnection;

use stariver_core::application::blog_service::ArticleApplication;

/// 应用的各个状态
pub struct AppState {
    pub article_application: ArticleApplication,
}

impl AppState {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        let article_application = ArticleApplication::new(conn);
        AppState {
            article_application,
        }
    }
}
