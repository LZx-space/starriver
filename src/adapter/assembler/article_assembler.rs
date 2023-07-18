use chrono::Local;
use sea_orm::DatabaseConnection;

use crate::adapter::api::blog_model::ArticleCmd;
use crate::adapter::repository::article_repository::ArticleRepositoryImpl;
use crate::application::article_service::ArticleApplication;
use crate::domain::blog::article::Article;

pub fn article_application(conn: &DatabaseConnection) -> ArticleApplication<ArticleRepositoryImpl> {
    let repository_impl = ArticleRepositoryImpl { conn };
    ArticleApplication::new(repository_impl)
}

pub fn cmd_2_new_entity(cmd: ArticleCmd, author_id: String) -> Article {
    Article {
        id: 1,
        title: cmd.title,
        body: cmd.body,
        tags: vec![],
        author_id,
        create_at: Local::now(),
        modified_records: vec![],
    }
}

pub fn cmd_2_update_entity(cmd: ArticleCmd, id: i64, author_id: String) -> Article {
    Article {
        id,
        title: cmd.title,
        body: cmd.body,
        tags: vec![],
        author_id,
        create_at: Default::default(),
        modified_records: vec![],
    }
}
