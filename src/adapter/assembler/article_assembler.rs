use chrono::Local;
use uuid::Uuid;

use crate::adapter::api::blog_model::ArticleCmd;
use crate::domain::blog::aggregate::Article;
use crate::domain::blog::value_object::State::Draft;

pub fn cmd_2_new_entity(cmd: ArticleCmd, author_id: String) -> Article {
    Article {
        id: Uuid::new_v4(),
        title: cmd.title,
        body: cmd.body,
        state: Draft,
        author_id,
        create_at: Local::now(),
        modified_records: vec![],
    }
}

pub fn cmd_2_update_entity(cmd: ArticleCmd, mut to_update: Article) -> Article {
    to_update.title = cmd.title;
    to_update.body = cmd.body;
    to_update
}
