use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PageQuery {
    pub page: u64,

    pub page_size: u64,
}

/// 页数据结果
#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub page: u8,

    pub page_size: u8,

    pub record_total: u8,

    pub records: Vec<T>,
}
