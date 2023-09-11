use chrono::Local;
use uuid::Uuid;

use crate::adapter::api::blog_model::ArticleCmd;
use crate::domain::blog::aggregate::Article;

pub fn cmd_2_new_entity(cmd: ArticleCmd, author_id: String) -> Article {
    Article {
        id: Uuid::new_v4(),
        title: cmd.title,
        body: cmd.body,
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
        author_id,
        create_at: Default::default(),
        modified_records: vec![],
    }
}
