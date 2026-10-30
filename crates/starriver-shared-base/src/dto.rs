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
pub struct OffsetPageQuery {
    #[validate(range(min = 0))]
    pub page: u64,
    #[validate(range(min = 1, max = 20))]
    pub page_size: u64,
}

/// 偏移分页数据结果
#[derive(Clone, Debug, Serialize)]
pub struct OffsetPage<T: Serialize> {
    page: u64,
    page_size: u64,
    total_pages: u64,
    total_items: u64,
    items: Vec<T>,
}

impl<T: Serialize> OffsetPage<T> {
    pub fn new(page: u64, page_size: u64, total_items: u64, items: Vec<T>) -> Self {
        let mut total_pages = total_items / page_size;
        if !total_items.is_multiple_of(page_size) {
            total_pages += 1;
        }
        OffsetPage {
            page,
            page_size,
            total_pages,
            total_items,
            items,
        }
    }

    pub fn from_query(query: OffsetPageQuery, total_items: u64, items: Vec<T>) -> Self {
        Self::new(query.page, query.page_size, total_items, items)
    }
}

/// 游标分页查询
#[derive(Debug, Deserialize, Validate)]
pub struct CursorPageQuery {
    /// 首页不传，其余情况传响应中返回的 next_cursor 字段值
    pub cursor: Option<String>,
    #[validate(range(min = 1, max = u8::MAX))]
    #[serde(default = "default_limit")]
    pub limit: u8,
}

fn default_limit() -> u8 {
    20
}

/// 游标分页数据结果
#[derive(Clone, Debug, Serialize)]
pub struct CursorPage<T: Serialize> {
    next_cursor: Option<String>,
    limit: u8,
    items: Vec<T>,
}
