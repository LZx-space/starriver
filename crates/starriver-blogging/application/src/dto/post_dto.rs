pub mod req {
    use serde::Deserialize;
    use uuid::Uuid;
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
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
    pub struct UpdatePostCmd {
        #[validate(length(min = 1, max = 50))]
        pub title: String,
        #[validate(length(min = 1, max = 50000))]
        pub content: String,
        pub category_id: Uuid,
        #[validate(length(min = 0, max = 10))]
        pub attachment_ids: Vec<uuid::Uuid>,
        /// 是否发布
        pub publish: bool,
    }
}

pub mod res {

    use serde::Serialize;
    use starriver_shared_base::dto::IdName;
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct PostDetail {
        pub id: Uuid,
        pub title: String,
        pub content: String,
        pub state: i16,
        pub category: IdName<Uuid>,
        pub attachments: Vec<PostAttachment>,
        pub published_at: Option<OffsetDateTime>,
        pub created_at: OffsetDateTime,
        pub updated_at: Option<OffsetDateTime>,
    }

    #[derive(Serialize)]
    pub struct PostExcerpt {
        pub id: Uuid,
        pub title: String,
        pub excerpt: String,
        pub state: i16,
        pub category: String,
        pub published_at: Option<OffsetDateTime>,
        pub created_at: OffsetDateTime,
        pub updated_at: Option<OffsetDateTime>,
    }

    #[derive(Serialize)]
    pub struct PostAttachment {
        pub id: Uuid,
        pub file_name: String,
        pub url: String,
    }
}
