use sea_orm::DatabaseConnection;

use crate::application::blog_service::ArticleApplication;

/// 应用的各个状态
pub struct AppState {
    pub conn: &'static DatabaseConnection,
    pub article_application: ArticleApplication,
}

impl AppState {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        let article_application = ArticleApplication::new(conn);
        AppState {
            conn,
            article_application,
        }
    }
}
