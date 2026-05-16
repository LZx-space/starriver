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
    category_id: Option<Uuid>, //  none only when state is draft
    published_at: Option<OffsetDateTime>,
}

impl Post {
    pub fn new(
        title: Title,
        content: Content,
        state: PostState,
        author_id: Uuid,
        category_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            title,
            content,
            state,
            author_id,
            category_id,
            published_at: None,
        }
    }

    pub fn from_repo(
        id: Uuid,
        title: Title,
        content: Content,
        state: PostState,
        author_id: Uuid,
        category_id: Option<Uuid>,
        published_at: Option<OffsetDateTime>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            state,
            author_id,
            category_id,
            published_at,
        }
    }

    /// 创建一个空的草稿，关联作者ID，并且其自身ID需被用于附件等资源关联
    pub fn new_empty_draft(author_id: Uuid) -> Result<Self, DomainError> {
        let title = Title::draft();
        let content = Content::new(String::new())?;
        Ok(Self {
            id: Uuid::now_v7(),
            title,
            content,
            state: PostState::Draft,
            author_id,
            category_id: None,
            published_at: None,
        })
    }

    /// 非附件属性更新
    pub fn update(&mut self, update: PostUpdate) -> Result<(), DomainError> {
        self.title = Title::new(update.title)?;
        self.content = Content::new(update.content)?;
        self.category_id = Some(update.category_id);
        if update.published {
            self.publish()?;
        }
        Ok(())
    }

    /// 将博客状态设置为草稿，已发布等状态的也能设为草稿
    pub fn draft(&mut self) {
        self.state = PostState::Draft;
        self.category_id = None;
        self.published_at = None;
    }

    /// 发布博客，将状态从草稿变为已发布
    pub fn publish(&mut self) -> Result<(), DomainError> {
        if self.title.0.is_empty() {
            return Err(DomainError::PostTitleIsEmpty);
        }
        if self.content.0.is_empty() {
            return Err(DomainError::PostContentIsEmpty);
        }
        if self.category_id().is_none() {
            return Err(DomainError::PostCategoryIsNone);
        }
        self.state = PostState::Published;
        self.published_at = Some(OffsetDateTime::now_utc());
        Ok(())
    }
}
