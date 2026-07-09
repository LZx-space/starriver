pub mod req {
    use serde::Deserialize;
    use uuid::Uuid;
    use validator::Validate;

    #[derive(Debug, Deserialize, PartialEq, Eq, Validate)]
    pub struct PageQuery {
        #[validate(range(min = 0, max = 10))]
        pub page: u64,
        #[validate(range(min = 1, max = 20))]
        pub page_size: u64,
        /// some， when need to filter by category
        pub category_id: Option<Uuid>,
        #[serde(default)]
        pub published_only: bool,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct SearchQuery {
        #[validate(length(min = 1, max = 10))]
        pub q: String,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct SaveOrUpdatePostCmd {
        #[validate(length(min = 1, max = 50))]
        pub title: String,
        #[validate(length(min = 1, max = 50000))]
        pub content: String,
        pub category_id: Uuid,
        pub attachments: Vec<Uuid>,
        /// 是否发布
        pub publish: bool,
    }
}

pub mod res {

    use serde::Serialize;
    use starriver_shared_base::dto::IdName;
    use time::OffsetDateTime;
    use uuid::Uuid;

    use crate::dto::attachment_dto::res::AttachmentDto;

    #[derive(Clone, Serialize)]
    pub struct PostDetailDto {
        pub id: Uuid,
        pub title: String,
        pub content: String,
        pub state: String,
        pub category: IdName<Uuid>,
        pub attachments: Vec<AttachmentDto>,
        pub published_at: Option<OffsetDateTime>,
        pub created_at: OffsetDateTime,
        pub updated_at: Option<OffsetDateTime>,
    }

    #[derive(Clone, Serialize)]
    pub struct PostExcerptDto {
        pub id: Uuid,
        pub title: String,
        pub excerpt: String,
        pub state: String,
        pub category: String,
        pub published_at: Option<OffsetDateTime>,
        pub created_at: OffsetDateTime,
        pub updated_at: Option<OffsetDateTime>,
    }

    #[derive(Clone, Serialize)]
    pub struct PostSearchDto {
        pub id: Uuid,
        pub title: String,
        pub snippet: String,
        pub category: String,
        pub published_at: OffsetDateTime,
        pub rank: f32,
    }
}
