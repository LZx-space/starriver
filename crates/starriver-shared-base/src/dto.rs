use std::marker::PhantomData;

use ::serde::{Deserialize, Serialize};
use serde::{Deserializer, de::DeserializeOwned};
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

#[derive(Debug)]
pub struct SignedCursor<T> {
    inner: String,
    _mark: PhantomData<T>,
}

impl<T: DeserializeOwned> SignedCursor<T> {
    /// 从已验签的字符串反序列化成目标游标结构体
    pub fn decode(&self) -> Result<T, String> {
        serde_json::from_slice::<T>(self.inner.as_bytes())
            .map_err(|e| format!("cursor json decode failed: {e}"))
    }
}

impl<T: Serialize> SignedCursor<T> {
    pub fn encode(v: &T) -> Result<String, String> {
        serde_json::to_string(v).map_err(|e| format!("cursor json decode failed: {e}"))
    }
}

/// 提取层只做验签：非法游标在这里变成 400
impl<'de, T> Deserialize<'de> for SignedCursor<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        // todo verify signature
        // raw cursor
        Ok(SignedCursor {
            inner: s,
            _mark: PhantomData,
        }) // 只存原文，不在这里解析成 T
    }
}

/// 游标分页查询
#[derive(Debug, Deserialize, Validate)]
pub struct CursorPageQuery<T> {
    /// 首页不传，其余情况传响应中返回的 next_cursor 字段值
    pub cursor: Option<SignedCursor<T>>,

    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_limit")]
    pub limit: u8,
}

fn default_limit() -> u8 {
    10
}

/// 游标分页数据结果
#[derive(Clone, Debug, Serialize)]
pub struct CursorPage<T: Serialize> {
    pub next_cursor: Option<String>,
    pub items: Vec<T>,
}
