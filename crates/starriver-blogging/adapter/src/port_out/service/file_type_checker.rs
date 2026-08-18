use starriver_blogging_domain::{
    attachment::file_type_checker::FileTypeChecker, shared_error::DomainError,
};

pub struct DefaultFileTypeChecker {}

impl FileTypeChecker for DefaultFileTypeChecker {
    const MAGIC_CHECKER_HEADER_SIZE: usize = 100;

    fn check(&self, header: &[u8], claimed_extension: &str) -> Result<bool, DomainError> {
        if let Some(file_type) = infer::get(header) {
            // infer 对同一格式只返回一个规范后缀（JPEG → "jpg"），
            // 归一化后比较，使 "jpeg" 与 "jpg"、大小写变体都视为匹配
            let claimed = claimed_extension.to_ascii_lowercase();
            let actual = file_type.extension();
            let same = actual == claimed
                || (actual == "jpg" && claimed == "jpeg")
                || (actual == "jpeg" && claimed == "jpg");
            Ok(same)
        } else {
            Ok(false)
        }
    }
}
