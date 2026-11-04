use std::collections::HashSet;

use derive_getters::{Dissolve, Getters};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    post::value_object::{Content, PostState, Title},
    shared_error::DomainError,
};

/// The post aggregate
#[derive(Clone, Debug, Getters, Dissolve, PartialEq)]
#[dissolve(rename = "into_parts")]
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
        author_id: Uuid,
        category_id: Uuid,
        attachments: Vec<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            title,
            content,
            state: PostState::Draft,
            author_id,
            category_id,
            attachments: dedup_attachments(attachments),
            published_at: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_persisted(
        id: Uuid,
        title: Title,
        content: Content,
        state: PostState,
        author_id: Uuid,
        category_id: Uuid,
        attachments: Vec<Uuid>,
        published_at: Option<OffsetDateTime>,
    ) -> Self {
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

    /// 更新标题、内容、分类与附件
    pub fn update(
        &mut self,
        title: Title,
        content: Content,
        category_id: Uuid,
        attachments: Vec<Uuid>,
    ) {
        self.title = title;
        self.content = content;
        self.category_id = category_id;
        self.attachments = dedup_attachments(attachments);
    }

    /// 将博客状态设置为草稿，已发布等状态的也能设为草稿
    /// - 将清除发布时间
    pub fn draft(&mut self) {
        match self.state {
            PostState::Draft => {}
            PostState::Published | PostState::Archived => {
                self.state = PostState::Draft;
                self.published_at = None;
            }
        }
    }

    /// 发布博客，将状态从草稿变为已发布
    /// - 已发布不会修改发布时间
    pub fn publish(&mut self, now: impl FnOnce() -> OffsetDateTime) -> Result<(), DomainError> {
        if self.title.0.is_empty() {
            return Err(DomainError::PostTitleIsEmpty);
        }
        if self.content.0.is_empty() {
            return Err(DomainError::PostContentIsEmpty);
        }
        match self.state {
            PostState::Published => Ok(()),
            PostState::Draft | PostState::Archived => {
                self.state = PostState::Published;
                self.published_at = Some(now());
                Ok(())
            }
        }
    }

    /// 提取正文摘要
    pub fn content_excerpt(&self) -> String {
        self.content.excerpt()
    }

    /// 正文剥离HTML标签
    pub fn content_strip_html_tags(&self) -> String {
        self.content.strip_html_tags()
    }
}

/// 附件列表是集合：重复 ID 会产生重复关联行并触发主键冲突
fn dedup_attachments(attachments: Vec<Uuid>) -> Vec<Uuid> {
    let mut seen = HashSet::with_capacity(attachments.len());
    attachments
        .into_iter()
        .filter(|id| seen.insert(*id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_deduplicates_attachments() {
        let attachment_id = Uuid::now_v7();
        let title = Title::new("title".into()).expect("");
        let content = Content::new("content".into()).expect("");
        let post = Post::new(
            title,
            content,
            Uuid::now_v7(),
            Uuid::now_v7(),
            vec![attachment_id, attachment_id],
        );

        assert_eq!(post.attachments(), &vec![attachment_id]);
    }
}
