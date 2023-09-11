use chrono::{DateTime, Local};
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::DbErr;
use uuid::Uuid;

use crate::adapter::api::blog_model::ArticleSummary;
use crate::domain::blog::value_object::Tag;
use crate::infrastructure::model::page::{PageQuery, PageResult};

/// 文章
#[derive(Debug)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub tags: Vec<Tag>,
    pub author_id: String,
    pub create_at: DateTime<Local>,
    pub modified_records: Vec<ModifiedRecord>,
}

impl Article {
    /// 验证数据
    #[allow(unused)]
    pub fn valid(&self) -> Result<bool, &str> {
        if self.title.trim().len() == 0 {
            return Err("标题不能为空");
        }
        if self.body.trim().len() == 0 {
            return Err("正文不能为空");
        }
        if self.tags.is_empty() {
            return Err("文章必须至少有一个标签");
        }
        use std::error::Error;
        Ok(true)
    }
}

/// 修改记录
#[derive(Debug)]
pub struct ModifiedRecord {
    pub id: Uuid,
    pub create_at: DateTime<Local>,
    pub modifier_id: String,
}

/// 仓库
#[async_trait]
pub trait ArticleRepository {
    /// 查询一页数据
    async fn find_page(&self, query: PageQuery) -> Result<PageResult<ArticleSummary>, DbErr>;

    /// 按ID查找
    async fn find_one(&self, id: Uuid) -> Result<Option<Article>, DbErr>;

    /// 新增
    async fn add(&self, e: Article) -> Result<bool, DbErr>;

    /// 删除
    async fn delete(&self, id: Uuid) -> Result<bool, DbErr>;

    /// 修改
    async fn update(&self, e: Article) -> Result<bool, DbErr>;
}
