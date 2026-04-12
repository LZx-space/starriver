use std::collections::HashSet;

use crate::article::{
    params::ArticleUpdate,
    value_object::{ArticleState, Content, Title},
};
use derive_getters::{Dissolve, Getters};
use starriver_infrastructure::error::ApiError;
use time::OffsetDateTime;
use uuid::Uuid;

/// The article aggregate
#[derive(Debug, Getters, Dissolve)]
pub struct Article {
    id: Uuid,
    title: Title,
    content: Content,
    state: ArticleState,
    #[getter(skip)]
    attachments: Vec<Attachment>,
    author_id: Uuid,
    create_at: OffsetDateTime,
    update_at: Option<OffsetDateTime>,
}

impl Article {
    pub fn new(
        title: Title,
        content: Content,
        state: ArticleState,
        attachments: Vec<Attachment>,
        author_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            title,
            content,
            state,
            attachments,
            author_id,
            create_at: OffsetDateTime::now_utc(),
            update_at: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_repo(
        id: Uuid,
        title: Title,
        content: Content,
        state: ArticleState,
        attachments: Vec<Attachment>,
        author_id: Uuid,
        create_at: OffsetDateTime,
        update_at: Option<OffsetDateTime>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            state,
            attachments,
            author_id,
            create_at,
            update_at,
        }
    }

    pub fn attachments(&mut self) -> &mut Vec<Attachment> {
        &mut self.attachments
    }

    /// 创建一个空的草稿，关联作者ID，并且其自身ID需被用于附件等资源关联
    pub fn new_empty_draft(author_id: Uuid) -> Result<Self, ApiError> {
        let title = Title::new(String::new())?;
        let content = Content::new(String::new())?;
        Ok(Self {
            id: Uuid::now_v7(),
            title,
            content,
            state: ArticleState::Draft,
            attachments: Vec::new(),
            author_id,
            create_at: OffsetDateTime::now_utc(),
            update_at: None,
        })
    }

    /// 添加附件
    pub fn add_attachment(&mut self, attachment: Attachment) -> Result<(), ApiError> {
        self.attachments.push(attachment);
        Ok(())
    }

    /// 非附件属性更新
    pub fn update(&mut self, update: ArticleUpdate) -> Result<(), ApiError> {
        self.title = Title::new(update.title)?;
        self.content = Content::new(update.content)?;
        self.update_at = Some(OffsetDateTime::now_utc());
        // 1. 提取新 ID 集合，用于快速判断
        let new_id_set: HashSet<_> = update.attachment_ids.iter().cloned().collect();
        // 2. 删除不在新 ID 集合中的附件
        self.attachments.retain(|att| new_id_set.contains(&att.id));
        // 3. 添加新 ID 对应的附件（如果尚未存在）
        for id in update.attachment_ids {
            if !self.attachments.iter().any(|att| att.id == id) {
                self.attachments.push(Attachment {
                    id,
                    article_id: self.id,
                    create_at: OffsetDateTime::now_utc(),
                    update_at: None,
                });
            }
        }
        if update.published {
            self.publish()?;
        }
        Ok(())
    }

    /// 将博客状态设置为草稿，已发布等状态的也能设为草稿
    pub fn draft(&mut self) {
        self.state = ArticleState::Draft;
    }

    /// 发布博客，将状态从草稿变为已发布
    pub fn publish(&mut self) -> Result<(), ApiError> {
        if self.title.0.is_empty() {
            return Err(ApiError::with_bad_request("title can't be empty"));
        }
        if self.content.0.is_empty() {
            return Err(ApiError::with_bad_request("content can't be empty"));
        }
        self.state = ArticleState::Published;
        Ok(())
    }

    /// 归档
    pub fn archive(&mut self) {
        self.state = ArticleState::Archived;
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Debug, Getters, Dissolve)]
pub struct Attachment {
    /// 作为文件名，这样无论文件存储位置如何变化都能通过配置文件定位到存储地址和保持URL不变
    id: Uuid,
    article_id: Uuid,
    create_at: OffsetDateTime,
    update_at: Option<OffsetDateTime>,
}

impl Attachment {
    pub fn new(article_id: Uuid) -> Self {
        Self {
            id: Uuid::now_v7(),
            article_id,
            create_at: OffsetDateTime::now_utc(),
            update_at: None,
        }
    }

    pub fn from_repo(
        id: Uuid,
        article_id: Uuid,
        create_at: OffsetDateTime,
        update_at: Option<OffsetDateTime>,
    ) -> Self {
        Self {
            id,
            article_id,
            create_at,
            update_at,
        }
    }

    pub fn filename(&self, extension: &str) -> String {
        format!("{}.{}", self.id, extension)
    }
}
