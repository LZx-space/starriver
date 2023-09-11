use sea_orm::error::DbErr;
use sea_orm::prelude::async_trait::async_trait;
use uuid::Uuid;

use crate::domain::tag::aggregate::Tag;
use crate::infrastructure::model::page::{PageQuery, PageResult};

/// 仓库
#[async_trait]
pub trait TagRepository {
    /// 查询一页数据
    async fn find_page(&self, query: PageQuery) -> Result<PageResult<Tag>, DbErr>;

    /// 按ID查找
    async fn find_one(&self, id: Uuid) -> Result<Option<Tag>, DbErr>;

    /// 新增
    async fn add(&self, e: Tag) -> Result<bool, DbErr>;

    /// 删除
    async fn delete(&self, id: Uuid) -> Result<bool, DbErr>;

    /// 修改
    async fn update(&self, e: Tag) -> Result<bool, DbErr>;
}
