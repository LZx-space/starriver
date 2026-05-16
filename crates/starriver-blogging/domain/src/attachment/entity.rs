use derive_getters::{Dissolve, Getters};
use uuid::Uuid;

use crate::attachment::value_object::{FileSize, MimeType};

#[derive(Clone, Debug, Getters, Dissolve)]
pub struct Attachment {
    /// 作为文件名，这样无论文件存储位置如何变化都能通过配置文件定位到存储地址和保持URL不变
    id: Uuid,
    file_name: String,
    file_size: u64,
}

impl Attachment {
    pub(crate) fn new(mime_type: MimeType, file_size: FileSize) -> Self {
        Self {
            id: Uuid::now_v7(),
            file_name: Uuid::now_v7().to_string() + "." + mime_type.as_str(),
            file_size: file_size.size,
        }
    }
}
