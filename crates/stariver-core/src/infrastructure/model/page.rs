use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: u64,

    pub page_size: u64,
}

/// 页数据结果
#[derive(Debug, Serialize)]
pub struct PageResult<T: Serialize> {
    page: u64,

    page_size: u64,

    total_pages: u64,

    total_items: u64,

    items: Vec<T>,
}

impl<T: Serialize> PageResult<T> {
    pub fn new(page: u64, page_size: u64, total_items: u64, items: Vec<T>) -> Self {
        PageResult {
            page,
            page_size,
            total_pages: 0,
            total_items,
            items,
        }
    }
}
