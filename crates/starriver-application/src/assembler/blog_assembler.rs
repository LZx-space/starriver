use time::OffsetDateTime;
use uuid::Uuid;

use starriver_domain::blog::entity::Blog;
use starriver_domain::blog::value_object::State::Draft;

use crate::blog_dto::BlogDetail;
use crate::dto::blog_dto::BlogCmd;

#[inline]
pub fn cmd_2_new_entity(cmd: BlogCmd, blogger_id: String) -> Blog {
    Blog {
        id: Uuid::now_v7(),
        title: cmd.title,
        body: cmd.body,
        state: Draft,
        author_id: blogger_id,
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
