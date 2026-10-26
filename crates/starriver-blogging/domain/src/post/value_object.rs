use std::fmt::{Display, Formatter};

use ammonia::clean;
use html_escape::{decode_html_entities, encode_text};
use striptags::strip_tags;

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

#[derive(Clone, Debug, PartialEq)]
pub struct Title(pub(crate) String);

impl Title {
    pub(crate) fn new(value: String) -> Result<Self, DomainError> {
        // 清洗会把 `<`/`&` 转义为实体，长度可能变长；必须对最终入库的字符串限长
        let value = clean(&value);
        if value.chars().count() > 50 {
            return Err(DomainError::PostTitleTooLong(value));
        }
        Ok(Self(value))
    }
}

impl Display for Title {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
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
        let no_tags = strip_tags(&decoded);
        // 转义剩余文本中的 <>&：摘要契约是“可直接按 HTML 渲染的文本”，
        // 防止解码残留（如双重编码 &amp;lt;）被浏览器二次解析成标签
        let escaped = encode_text(&no_tags);
        let len = escaped.chars().count();

        const TARGET_LEN: usize = 200;
        const MAX_EXCERPT_LEN: usize = 250; // 与 post.excerpt 列宽 varchar(250) 保持一致
        const ELLIPSIS: &str = "...";

        if len <= TARGET_LEN {
            return escaped.to_string();
        }

        // 文本长度大于TARGET_LEN，在 [TARGET_LEN, MAX_EXCERPT_LEN) 区间内查找第一个分隔符
        let mut end = TARGET_LEN;
        let search_limit = MAX_EXCERPT_LEN.min(len);
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

        // 如果没找到分隔符，则截断到 TARGET_LEN
        if end == search_limit {
            end = TARGET_LEN;
        }

        // 截断内容加上省略号不得超过列宽，否则合法内容写库会报错
        let end = end.min(MAX_EXCERPT_LEN - ELLIPSIS.len());

        format!("{}{}", chars[..end].iter().collect::<String>(), ELLIPSIS)
    }

    pub(crate) fn strip_html_tags(&self) -> String {
        strip_tags(&self.0)
    }
}

#[cfg(test)]
mod title_tests {
    use super::*;

    #[test]
    fn title_length_is_checked_after_sanitizing() {
        // ammonia 会把每个 `<` 转义为 `&lt;`，清洗后长度为原来的 4 倍，
        // 若在清洗前校验，这串 50 字符的标题会突破 varchar(50) 列宽。
        let oversized = "<".repeat(50);
        assert!(matches!(
            Title::new(oversized),
            Err(DomainError::PostTitleTooLong(_))
        ));
    }

    #[test]
    fn sanitized_title_at_limit_is_accepted() {
        let title = Title::new("a".repeat(50)).expect("50 chars fits column width");
        assert_eq!(title.0.chars().count(), 50);
    }
}

#[cfg(test)]
mod excerpt_tests {
    use super::*;

    #[test]
    fn excerpt_with_delimiter_fits_column_width() {
        // 分隔符位于可搜索区间内：247 字符 + "..." = 250，恰好等于列宽
        let content = format!("{}。{}", "a".repeat(247), "b".repeat(20));
        let excerpt = Content::new(content).expect("valid content").excerpt();
        assert_eq!(excerpt.chars().count(), 250);
    }

    #[test]
    fn excerpt_beyond_column_width_is_capped() {
        // 分隔符位于 249：旧实现会产出 252 字符，超出 varchar(250)
        let content = format!("{}.", "a".repeat(249));
        let excerpt = Content::new(content).expect("valid content").excerpt();
        assert!(excerpt.chars().count() <= 250);
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}
