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

pub fn cmd_2_new_entity(cmd: ArticleCmd, creator_id: String) -> Article {
    Article {
        id: Uuid::new_v4(),
        title: cmd.title,
        body: cmd.body,
        tags: Default::default(),
        creator_id,
        create_time: Local::now(),
        modified_records: vec![],
    }
}

pub fn cmd_2_update_entity(cmd: ArticleCmd, id: Uuid, creator_id: String) -> Article {
    Article {
        id,
        title: cmd.title,
        body: cmd.body,
        tags: Default::default(),
        creator_id,
        create_time: Default::default(),
        modified_records: vec![],
    }
}
