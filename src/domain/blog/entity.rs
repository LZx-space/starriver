use chrono::{DateTime, Local};
use uuid::Uuid;

/// 修改记录
#[derive(Debug)]
pub struct ModifiedRecord {
    id: Uuid,

    datetime: DateTime<Local>,

    modifier: String,
}
