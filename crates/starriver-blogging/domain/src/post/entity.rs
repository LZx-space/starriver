use derive_getters::{Dissolve, Getters};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    post::{
        params::PostUpdate,
        value_object::{Content, PostState, Title},
    },
    shared_error::DomainError,
};

/// The post aggregate
#[derive(Clone, Debug, Getters, Dissolve)]
pub struct Post {
    id: Uuid,
    title: Title,
    content: Content,
    state: PostState,
    author_id: Uuid,
    category_id: Uuid,
    attachments: Vec<Uuid>,
    published_at: Option<OffsetDateTime>,
}

impl Post {
    pub fn new(
        title: Title,
        content: Content,
        state: PostState,
        author_id: Uuid,
        category_id: Uuid,
        attachments: Vec<Uuid>,
    ) -> Result<Self, DomainError> {
        let published_at = match state {
            PostState::Draft => None,
            PostState::Published => Some(OffsetDateTime::now_utc()),
            PostState::Archived => None,
        };
        Ok(Self {
            id: Uuid::now_v7(),
            title,
            content,
            state,
            author_id,
            category_id,
            attachments,
            published_at,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_repo(
        id: Uuid,
        title: String,
        content: String,
        state: PostState,
        author_id: Uuid,
        category_id: Uuid,
        attachments: Vec<Uuid>,
        published_at: Option<OffsetDateTime>,
    ) -> Self {
        let title = Title(title);
        let content = Content(content);
        Self {
            id,
            title,
            content,
            state,
            author_id,
            category_id,
            attachments,
            published_at,
        }
    }

    /// 非附件属性更新
    pub fn update(&mut self, update: PostUpdate) -> Result<(), DomainError> {
        self.title = Title::new(update.title)?;
        self.content = Content::new(update.content)?;
        self.category_id = update.category_id;
        self.attachments = update.attachments;
        if update.published {
            self.publish()?;
        } else {
            self.draft();
        }
        Ok(())
    }

    /// 将博客状态设置为草稿，已发布等状态的也能设为草稿
    pub fn draft(&mut self) {
        if matches!(self.state, PostState::Draft) {
            return; // 已是草稿，不重复清 published_at
        }
        self.state = PostState::Draft;
        self.published_at = None;
    }

    /// 发布博客，将状态从草稿变为已发布
    pub fn publish(&mut self) -> Result<(), DomainError> {
        if matches!(self.state, PostState::Published) {
            return Ok(()); // 幂等，不重置 published_at
        }
        if self.title.0.is_empty() {
            return Err(DomainError::PostTitleIsEmpty);
        }
        if self.content.0.is_empty() {
            return Err(DomainError::PostContentIsEmpty);
        }
        self.state = PostState::Published;
        self.published_at = Some(OffsetDateTime::now_utc());
        Ok(())
    }
}
