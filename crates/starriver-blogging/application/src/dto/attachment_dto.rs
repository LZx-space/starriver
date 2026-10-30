pub mod output {
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Clone, Serialize)]
    pub struct AttachmentDto {
        pub id: Uuid,
        pub file_name: String,
        pub url: String,
    }
}
