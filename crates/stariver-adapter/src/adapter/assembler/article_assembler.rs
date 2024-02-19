use chrono::Local;
use uuid::{NoContext, Timestamp, Uuid};

use crate::adapter::api::blog_model::ArticleCmd;
use stariver_core::domain::blog::aggregate::Article;
use stariver_core::domain::blog::value_object::State::Draft;

pub fn cmd_2_new_entity(cmd: ArticleCmd, author_id: String) -> Article {
    Article {
        id: Uuid::new_v7(Timestamp::now(NoContext)),
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
