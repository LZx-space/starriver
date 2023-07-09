use chrono::Local;
use sea_orm::prelude::Uuid;
use sea_orm::DatabaseConnection;

use crate::adapter::api::blog_model::ArticleCmd;
use crate::adapter::repository::article_repository::ArticleRepositoryImpl;
use crate::application::article_service::ArticleApplication;
use crate::domain::article::Article;

pub fn article_application(conn: &DatabaseConnection) -> ArticleApplication<ArticleRepositoryImpl> {
    let repository_impl = ArticleRepositoryImpl { conn };
    ArticleApplication::new(repository_impl)
}

pub fn cmd_2_new_entity(cmd: ArticleCmd, author_id: String) -> Article {
    Article {
        id: Uuid::new_v4(),
        title: cmd.title,
        body: cmd.body,
        tags: Default::default(),
        author_id,
        create_at: Local::now(),
        modified_records: vec![],
    }
}

pub fn cmd_2_update_entity(cmd: ArticleCmd, id: Uuid, author_id: String) -> Article {
    Article {
        id,
        title: cmd.title,
        body: cmd.body,
        tags: Default::default(),
        author_id,
        create_at: Default::default(),
        modified_records: vec![],
    }
}
