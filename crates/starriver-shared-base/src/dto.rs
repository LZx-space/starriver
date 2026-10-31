use std::marker::PhantomData;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error;
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
    #[serde(deserialize_with = "deserialize_u64_from_str")]
    pub page: u64,
    #[validate(range(min = 1, max = 20))]
    #[serde(deserialize_with = "deserialize_u64_from_str")]
    pub page_size: u64,
}

fn deserialize_u64_from_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
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

/// 签名的游标
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Signed<T> {
    cursor: String,
    #[serde(skip)]
    _mark: PhantomData<T>,
}

impl<T> Signed<T> {
    pub fn as_str(&self) -> &str {
        self.cursor.as_str()
    }
}

impl<T: Serialize> Signed<T> {
    pub fn encode(value: &T) -> Result<Self, CursorError> {
        let payload = serde_json::to_string(value)
            .map_err(|e| CursorError::SerializationError(e.to_string()))?;
        let sig = "123";
        let payload = URL_SAFE_NO_PAD.encode(payload.as_bytes());
        let cursor = format!("{payload}.{sig}");
        Ok(Self {
            cursor,
            _mark: PhantomData,
        })
    }
}

impl<T: DeserializeOwned> Signed<T> {
    pub fn decode(&self) -> Result<T, CursorError> {
        let (payload, _sig) = self
            .cursor
            .rsplit_once('.')
            .ok_or(CursorError::Malformed("without signature".to_string()))?;
        let payload = URL_SAFE_NO_PAD
            .decode(payload)
            .map_err(|e| CursorError::Malformed(e.to_string()))?;
        // let key = Key::from_str("123")?;
        // key.verify(payload.as_bytes(), sig)?;
        Ok(serde_json::from_slice(&payload)
            .map_err(|e| CursorError::DeserializationError(e.to_string())))?
    }
}

#[derive(Debug, Error)]
pub enum CursorError {
    #[error("Cursor is malformed: {0}")]
    Malformed(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Signature error: {0}")]
    SignatureError(String),
    #[error("Signature verification error: {0}")]
    SignatureVerificationError(String),
}

/// 游标分页查询
#[derive(Deserialize, Validate)]
#[serde(bound(deserialize = "T: DeserializeOwned"))]
pub struct CursorPageQuery<T> {
    /// 首页不传，其余情况传响应中返回的 next_cursor 字段值
    pub cursor: Option<Signed<T>>,

    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_limit")]
    #[serde(deserialize_with = "deserialize_u64_from_str")]
    pub limit: u64,
}

fn default_limit() -> u64 {
    10
}

/// 游标分页数据结果
/// # Generic Types
/// - C: 被编码为游标字符串的类型
/// - T: 数据类型
#[derive(Serialize)]
pub struct CursorPage<C: Serialize, T: Serialize> {
    pub next_cursor: Option<Signed<C>>,
    pub items: Vec<T>,
}
