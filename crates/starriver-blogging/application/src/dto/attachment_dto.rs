pub mod req {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct UploadAttachmentCmd {
        pub file_name: String,
        pub claimed_extension: String,
        pub bytes: Vec<u8>,
    }
}

pub mod res {
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct PostAttachmentDto {
        pub id: Uuid,
        pub file_name: String,
        pub url: String,
    }
}
