use time::OffsetDateTime;
use uuid::Uuid;

use starriver_domain::blog::entity::Blog;
use starriver_domain::blog::value_object::BlogState::Draft;

use crate::blog_dto::{BlogCmd, BlogDetail};

#[inline]
pub fn cmd_2_new_entity(author_id: Uuid, cmd: BlogCmd) -> Blog {
    Blog {
        id: Uuid::now_v7(),
        title: cmd.title,
        body: cmd.body,
        state: Draft,
        author_id: author_id,
        create_at: OffsetDateTime::now_utc(),
        update_at: None,
    }
}

#[inline]
pub fn cmd_2_update_entity(cmd: BlogCmd, mut to_update: Blog) -> Blog {
    to_update.title = cmd.title;
    to_update.body = cmd.body;
    to_update
}

#[inline]
pub fn entity_2_vo(entity: Blog) -> BlogDetail {
    BlogDetail {
        title: entity.title,
        body: entity.body,
        state: entity.state.to_string(),
    }
}
