use std::marker::PhantomData;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::signature::{sign, verify};

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

///// Offset Pagination /////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Validate)]
pub struct OffsetPageQuery {
    #[validate(range(min = 0, max = 10000))]
    #[serde(deserialize_with = "deserialize_u64_from_str")]
    pub page: u64,
    #[validate(range(min = 1, max = 20))]
    #[serde(deserialize_with = "deserialize_u64_from_str")]
    pub page_size: u64,
}

/// 偏移分页数据结果
#[derive(Clone, Debug, Serialize)]
pub struct OffsetPage<T> {
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
        Self {
            page,
            page_size,
            total_pages,
            total_items,
            items,
        }
    }

    pub fn from_query(query: &OffsetPageQuery, total_items: u64, items: Vec<T>) -> Self {
        Self::new(query.page, query.page_size, total_items, items)
    }
}

///// Cursor Pagination /////////////////////////////////////////////////////////////////

/// 分页查询的游标
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Cursor<T> {
    encoded: String,
    #[serde(skip)]
    _mark: PhantomData<T>,
}

impl<T: Serialize> Cursor<T> {
    pub fn encode(key: &[u8], value: &T) -> Result<Self, CursorError> {
        let payload = serde_json::to_string(value)
            .map_err(|e| CursorError::SerializationError(e.to_string()))?;
        let sig = sign(key, payload.as_bytes());
        let sig = URL_SAFE_NO_PAD.encode(&sig);
        let payload = URL_SAFE_NO_PAD.encode(payload.as_bytes());
        let encoded = format!("{payload}.{sig}");
        Ok(Self {
            encoded,
            _mark: PhantomData,
        })
    }
}

impl<T: DeserializeOwned> Cursor<T> {
    fn decode(&self, key: &[u8]) -> Result<T, CursorError> {
        let (payload, sig) = self
            .encoded
            .split_once('.')
            .ok_or_else(|| CursorError::Malformed("without signature".to_string()))?;
        let payload = URL_SAFE_NO_PAD
            .decode(payload)
            .map_err(|e| CursorError::Malformed(e.to_string()))?;
        let sig = URL_SAFE_NO_PAD
            .decode(sig)
            .map_err(|e| CursorError::Malformed(e.to_string()))?;
        if !verify(key, &payload, &sig) {
            return Err(CursorError::SignatureVerificationError);
        }
        Ok(serde_json::from_slice(&payload)
            .map_err(|e| CursorError::DeserializationError(e.to_string())))?
    }
}

/// 游标分页查询
#[derive(Deserialize, Validate)]
pub struct CursorPageQuery<T> {
    /// 首页不传，其余情况传响应中返回的`next_cursor`字段值
    pub cursor: Option<Cursor<T>>,

    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_limit")]
    #[serde(deserialize_with = "deserialize_u64_from_str")]
    pub limit: u64,
}

impl<T: DeserializeOwned> CursorPageQuery<T> {
    pub fn decode_cursor(&self, key: &[u8]) -> Result<Option<T>, CursorError> {
        self.cursor.as_ref().map(|c| c.decode(key)).transpose()
    }
}

/// 游标分页数据结果
/// # Generic Types
/// - C: 被编码为游标字符串的类型
/// - T: 数据类型
#[derive(Serialize)]
pub struct CursorPage<C, T> {
    /// `Some(cursor)` if there is a next page
    pub next_cursor: Option<Cursor<C>>,
    pub items: Vec<T>,
}

#[derive(Debug, Error)]
pub enum CursorError {
    #[error("Malformed cursor: {0}")]
    Malformed(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Signature error: {0}")]
    SignatureError(String),
    #[error("Signature verification error")]
    SignatureVerificationError,
}

///// Pagination Common ////////////////////////////////////////////////////////////////////////////

fn deserialize_u64_from_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

const fn default_limit() -> u64 {
    10
}

/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
pub struct Range<T> {
    /// The lower bound of the range (inclusive).
    pub from: T,
    /// The upper bound of the range (exclusive).
    pub to: T,
}

pub trait RangeBound {
    fn is_valid_range(from: &Self, to: &Self) -> bool;
}

impl<T> Validate for Range<T>
where
    T: RangeBound,
{
    fn validate(&self) -> Result<(), ValidationErrors> {
        if T::is_valid_range(&self.from, &self.to) {
            Ok(())
        } else {
            let mut errors = ValidationErrors::new();
            errors.add(
                "from",
                ValidationError::new("invalid_range, `form` cannot be greater or equal to `to`"),
            );
            errors.add(
                "to",
                ValidationError::new("invalid_range, `to` cannot be less or equal than `from`"),
            );
            Err(errors)
        }
    }
}

impl RangeBound for Option<usize> {
    fn is_valid_range(from: &Self, to: &Self) -> bool {
        match (from, to) {
            (Some(a), Some(b)) => a < b,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_validate() {
        let r1 = Range {
            from: None,
            to: Some(0),
        };
        assert!(r1.validate().is_ok());

        let r2 = Range {
            from: Some(0),
            to: None,
        };
        assert!(r2.validate().is_ok());

        let r3 = Range {
            from: Some(0),
            to: Some(1),
        };
        assert!(r3.validate().is_ok());

        let r4 = Range {
            from: None,
            to: None,
        };
        assert!(r4.validate().is_ok());
    }
}
