use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize)]
pub struct IdName<T> {
    pub id: T,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct IdValue<I, V> {
    pub id: I,
    pub value: V,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PageQuery {
    #[validate(range(min = 0, max = 10))]
    pub page: u64,
    #[validate(range(min = 1, max = 20))]
    pub page_size: u64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PageSearch {
    #[validate(range(min = 0, max = 10))]
    pub page: u64,
    #[validate(range(min = 1, max = 20))]
    pub page_size: u64,
    #[validate(length(min = 1))]
    pub q: String,
}

/// 页数据结果
#[derive(Clone, Debug, Serialize)]
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
