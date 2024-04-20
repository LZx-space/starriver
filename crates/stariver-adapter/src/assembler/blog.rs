use chrono::Local;
use uuid::Uuid;

use stariver_core::domain::blog::aggregate::Article;
use stariver_core::domain::blog::value_object::State::Draft;

use crate::model::blog::ArticleCmd;

pub fn cmd_2_new_entity(cmd: ArticleCmd, author_id: String) -> Article {
    Article {
        id: Uuid::now_v7(),
        title: cmd.title,
        body: cmd.body,
        state: Draft,
        author_id,
        create_at: Local::now(),
        update_at: None,
    }
}

pub fn cmd_2_update_entity(cmd: ArticleCmd, mut to_update: Article) -> Article {
    to_update.title = cmd.title;
    to_update.body = cmd.body;
    to_update
}
