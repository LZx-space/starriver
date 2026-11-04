use derive_getters::Dissolve;
use uuid::Uuid;

use crate::attachment::value_object::{Extension, FileSize};

#[derive(Clone, Dissolve)]
#[dissolve(rename = "into_parts")]
pub struct Attachment {
    /// 作为文件名，这样无论文件存储位置如何变化都能通过配置文件定位到存储地址和保持URL不变
    id: Uuid,
    extension: Extension,
    file_size: FileSize,
}

impl Attachment {
    /// 虽然和`from_persisted`一样，但是还是都保留以提示开发注意重建和新建的概念
    pub(crate) fn new(id: Uuid, extension: Extension, file_size: FileSize) -> Self {
        Self {
            id,
            extension,
            file_size,
        }
    }

    pub fn from_persisted(id: Uuid, extension: Extension, file_size: FileSize) -> Self {
        Self {
            id,
            extension,
            file_size,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn file_name(&self) -> String {
        Attachment::generate_file_name(&self.id, &self.extension)
    }

    pub fn file_size(&self) -> i64 {
        self.file_size.size()
    }

    /// 命名规则的单点来源：`{id}.{extension}`
    pub fn generate_file_name(id: &Uuid, extension: &Extension) -> String {
        format!("{}.{}", id, extension.as_str())
    }
}
