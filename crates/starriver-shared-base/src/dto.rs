use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IdName<T> {
    pub id: T,
    pub name: String,
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
        let mut total_pages = total_items / page_size;
        if !total_items.is_multiple_of(page_size) {
            total_pages += 1;
        }
        PageResult {
            page,
            page_size,
            total_pages,
            total_items,
            items,
        }
    }
}
