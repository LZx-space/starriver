use chrono::{DateTime, Local};

/// 修改记录
#[derive(Debug, PartialEq, Eq)]
pub struct ModifiedRecord {
    id: i64,

    datetime: DateTime<Local>,

    modifier: String,
}
