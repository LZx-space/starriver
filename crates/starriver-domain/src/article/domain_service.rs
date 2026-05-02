pub struct AttachmentService;

impl AttachmentService {
    /// # Params
    /// - `proxy_prefix`: Web server proxy url prefix (e.g., "/uploads")
    /// - `file_name`: File name including extension
    pub fn access_url(proxy_prefix: &str, file_name: &str) -> String {
        format!("{}/{}", proxy_prefix, file_name)
    }
}
