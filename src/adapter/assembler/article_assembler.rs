use crate::adapter::api::blog_model::ArticleCmd;
use crate::adapter::repository::article_po::Model as ArticlePo;
use crate::domain::article::Article;
use chrono::Local;
use sea_orm::prelude::Uuid;

pub fn from_entity(entity: Article) -> Result<ArticlePo, &'static str> {
    Err("123")
}

pub fn form_po(po: ArticlePo) -> Result<Article, &'static str> {
    Err("222")
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
