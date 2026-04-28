use uuid::Uuid;

pub struct AttachmentService;

impl AttachmentService {
    pub fn file_name(id: &Uuid, extension: &str) -> String {
        format!("{}.{}", id, extension)
    }

    pub fn access_url(base_url: &str, file_name: &str) -> String {
        format!("/{}/{}", base_url, file_name)
    }
}
