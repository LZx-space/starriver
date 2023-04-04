use chrono::{DateTime, Local};
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr};
use uuid::Uuid;

use crate::domain::tag::Tag;
use crate::infrastructure::article_page_item::ArticlePageItem;
use crate::infrastructure::page::{PageQuery, PageResult};

/// 文章
#[derive(Debug)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub tags: Vec<Tag>,
    pub creator_id: String,
    pub create_time: DateTime<Local>,
    pub modified_records: Vec<ModifiedRecord>,
}

impl Article {
    /// 验证数据
    pub fn valid(&self) -> Result<bool, String> {
        println!("标题-{}", self.title);
        Ok(true)
    }
}

/// 修改记录
#[derive(Debug)]
pub struct ModifiedRecord {
    pub id: Uuid,
    pub datetime: DateTime<Local>,
    pub modifier_id: String,
}

/// 仓库
#[async_trait]
pub trait ArticleRepository {
    /// 查询一页数据
    async fn find_page(&self, query: PageQuery) -> Result<PageResult<ArticlePageItem>, DbErr>;

    /// 按ID查找
    async fn find_one(&self, id: Uuid) -> Result<Option<Article>, DbErr>;

    /// 新增
    async fn add(&self, e: Article) -> Result<bool, DbErr>;

    /// 删除
    async fn delete(&self, id: Uuid) -> Result<bool, DbErr>;

    /// 修改
    async fn update(&self, e: Article) -> Result<bool, DbErr>;
}
