use chrono::{DateTime, Local};
use uuid::Uuid;

use crate::domain::blog::entity::ModifiedRecord;

/// 文章
#[derive(Debug)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub body: String,
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
        Ok(true)
    }
}
