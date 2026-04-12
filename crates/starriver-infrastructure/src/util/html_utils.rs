/// 摘录生成器
pub trait Excerptor {
    /// 从内容中摘录指定数量的字符（不是单词）
    fn excerpt(content: &str, max_len: usize) -> String;
}
pub struct DefaultExcerptor {}
impl Excerptor for DefaultExcerptor {
    fn excerpt(content: &str, max_len: usize) -> String {
        if max_len == 0 {
            return String::new();
        }

        // 如果原文长度（字符数）小于等于限制，直接返回
        let char_count = content.chars().count();
        if char_count <= max_len {
            return content.to_string();
        }

        let mut result = String::new();
        let mut current_len = 0;

        // 按空白字符分割（会丢失连续空白，但对摘录足够）
        for word in content.split_whitespace() {
            let word_len = word.chars().count();
            // 尝试加上这个单词（以及前面的空格，除了第一个单词）
            let needed = if result.is_empty() {
                word_len
            } else {
                word_len + 1
            };

            if current_len + needed <= max_len {
                if !result.is_empty() {
                    result.push(' ');
                    current_len += 1;
                }
                result.push_str(word);
                current_len += word_len;
            } else {
                // 无法再添加完整单词，结束循环
                break;
            }
        }

        // 如果结果为空（例如第一个单词就超过 max_len），至少截取第一个单词的前 max_len 个字符
        if result.is_empty() {
            let truncated: String = content.chars().take(max_len).collect();
            return truncated;
        }

        result.push_str("...");
        result
    }
}
