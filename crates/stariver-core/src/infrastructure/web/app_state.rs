use sea_orm::DatabaseConnection;

use crate::application::blog_service::ArticleApplication;
use crate::application::user_service::UserApplication;
use crate::infrastructure::service::dictionary::dictionary_service::Dictionary;

/// 应用的各个状态
pub struct AppState {
    pub conn: &'static DatabaseConnection,
    pub article_application: ArticleApplication,
    pub user_application: UserApplication,
    pub dictionary: Dictionary,
}

impl AppState {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        let article_application = ArticleApplication::new(conn);
        let user_application = UserApplication::new(conn);
        let dictionary = Dictionary::new(conn);
        AppState {
            conn,
            article_application,
            user_application,
            dictionary,
        }
    }
}
