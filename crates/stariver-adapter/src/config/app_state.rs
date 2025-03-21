use sea_orm::DatabaseConnection;
use stariver_application::service::blog_service::BlogApplication;
use stariver_application::service::user_service::UserApplication;
use stariver_infrastructure::service::dictionary::dictionary_service::Dictionary;

/// 应用的各个状态
pub struct AppState {
    pub conn: &'static DatabaseConnection,
    pub user_application: UserApplication,
    pub blog_application: BlogApplication,
    pub dictionary: Dictionary,
}

impl AppState {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        let user_application = UserApplication::new(conn);
        let blog_application = BlogApplication::new(conn);
        let dictionary = Dictionary::new(conn);
        AppState {
            conn,
            user_application,
            blog_application,
            dictionary,
        }
    }
}
