use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: u64,

    pub page_size: u64,
}

/// 页数据结果
#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub page: u64,

    pub page_size: u64,

    pub record_total: u64,

    pub records: Vec<T>,
}
