use anyhow::Error;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::blog::value_object::State;
use crate::blog::value_object::State::{Draft, Released};

/// The blog aggregate. Blog is the aggregate root.
#[derive(Debug, Serialize)]
pub struct Blog {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub state: State,
    pub blogger_id: String,
    pub create_at: OffsetDateTime,
    pub update_at: Option<OffsetDateTime>,
}

impl Blog {
    /// 验证数据
    #[allow(unused)]
    pub fn valid(&self) -> Result<bool, Error> {
        if self.title.trim().is_empty() {
            return Err(Error::msg("标题不能为空"));
        }
        if self.body.trim().is_empty() {
            return Err(Error::msg("正文不能为空"));
        }
        Ok(true)
    }

    /// 进去到下一个状态
    pub fn next_state(&mut self) {
        if self.state.eq(&Draft) {
            self.state = Released;
        }
    }
}
