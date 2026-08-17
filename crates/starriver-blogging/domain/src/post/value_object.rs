use std::{
    fmt::{Display, Formatter},
    sync::LazyLock,
};

use ammonia::clean;
use html_escape::{decode_html_entities, encode_text};
use regex::Regex;

use crate::shared_error::DomainError;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub enum PostState {
    #[default]
    Draft,
    Published,
    Archived,
}

impl Display for PostState {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            PostState::Draft => f.write_str("draft"),
            PostState::Published => f.write_str("published"),
            PostState::Archived => f.write_str("archived"),
        }
    }
}

//////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Title(pub(crate) String);

impl Title {
    pub(crate) fn new(value: String) -> Result<Self, DomainError> {
        if value.chars().count() > 50 {
            return Err(DomainError::PostTitleTooLong(value));
        }
        let value = clean(&value);
        Ok(Self(value))
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

static TAG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<[^>]*>").expect("html tag regex error"));

#[derive(Clone, Debug)]
pub struct Content(pub(crate) String);

impl Content {
    pub(crate) fn new(value: String) -> Result<Self, DomainError> {
        if value.chars().count() > 50000 {
            return Err(DomainError::PostContentTooLong(value));
        }
        let value = clean(&value);
        Ok(Self(value))
    }

    pub(crate) fn excerpt(&self) -> String {
        // 先解码实体：
        // 实体化的标签（如 &lt;script&gt;）必须先还原成标签，才能被下一步剥掉。
        // 若先剥标签，实体化的标签会逃过正则、解码后又还原成可执行标签（存储型 XSS）。
        let decoded = decode_html_entities(&self.0);
        // 再剥标签
        let no_tags = TAG_REGEX.replace_all(&decoded, "");
        // 转义剩余文本中的 <>&：摘要契约是“可直接按 HTML 渲染的文本”，
        // 防止解码残留（如双重编码 &amp;lt;）被浏览器二次解析成标签
        let escaped = encode_text(&no_tags);
        let len = escaped.chars().count();

        const MAX_LEN: usize = 200;
        const EXTRA_SEARCH: usize = 50; // 允许向后多找50个字符的边界

        if len <= MAX_LEN {
            return escaped.to_string();
        }

        // 在 [MAX_LEN, MAX_LEN + EXTRA_SEARCH] 区间内查找第一个分隔符
        let mut end = MAX_LEN;
        let search_limit = (MAX_LEN + EXTRA_SEARCH).min(len);
        let chars: Vec<char> = escaped.chars().collect();

        while end < search_limit {
            let c = chars[end];
            // 英文
            if c == '.' || c == '!' || c == '?' {
                break;
            }
            // 中文
            if c == '。' || c == '！' || c == '？' {
                break;
            }
            // 其它
            if c == ' ' {
                break;
            }
            end += 1;
        }

        // 如果没找到，则截断到 MAX_LEN
        if end == search_limit {
            end = MAX_LEN;
        }

        format!("{}...", chars[..end].iter().collect::<String>())
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}
