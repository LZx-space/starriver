use derive_getters::Dissolve;
use uuid::Uuid;

use crate::attachment::value_object::{Extension, FileSize};

#[derive(Clone, Dissolve)]
pub struct Attachment {
    /// 作为文件名，这样无论文件存储位置如何变化都能通过配置文件定位到存储地址和保持URL不变
    id: Uuid,
    extension: Extension,
    file_size: FileSize,
}

impl Attachment {
    pub(crate) fn new(id: Uuid, extension: Extension, file_size: FileSize) -> Self {
        Self {
            id,
            extension,
            file_size,
        }
    }

    pub fn from_repo(id: Uuid, file_name: String, file_size: i64) -> Self {
        let ext = file_name.split(".").last().unwrap_or_default().to_string();
        let extension = Extension(ext);
        let file_size = FileSize(file_size);
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
        Attachment::make_file_name(&self.id, &self.extension)
    }

    pub fn file_size(&self) -> i64 {
        self.file_size.size()
    }

    /// 命名规则的单点来源：`{id}.{extension}`
    pub fn make_file_name(id: &Uuid, extension: &Extension) -> String {
        format!("{}.{}", id, extension.as_str())
    }
}
