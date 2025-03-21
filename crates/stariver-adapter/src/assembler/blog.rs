use time::OffsetDateTime;
use uuid::Uuid;

use stariver_domain::blog::aggregate::Blog;
use stariver_domain::blog::value_object::State::Draft;

use crate::model::blog::BlogCmd;

pub fn cmd_2_new_entity(cmd: BlogCmd, blogger_id: String) -> Blog {
    Blog {
        id: Uuid::now_v7(),
        title: cmd.title,
        body: cmd.body,
        state: Draft,
        blogger_id,
        create_at: OffsetDateTime::now_utc(),
        update_at: None,
    }
}

pub fn cmd_2_update_entity(cmd: BlogCmd, mut to_update: Blog) -> Blog {
    to_update.title = cmd.title;
    to_update.body = cmd.body;
    to_update
}
