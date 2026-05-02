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

        /// some when need to filter by category
        pub category_id: Option<Uuid>,

        #[serde(default)]
        pub published_only: bool,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct UpdateArticleCmd {
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

    use crate::model::dto::IdName;
    use sea_orm::FromQueryResult;
    use serde::Serialize;
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(Serialize, FromQueryResult)]
    pub struct ArticleDetail {
        #[sea_orm(from_alias = "article_id")]
        pub id: Uuid,

        pub title: String,

        pub content: String,

        pub state: i16,

        /// none when state is draft
        #[sea_orm(nested)]
        pub category: Option<IdName<Uuid>>,

        #[sea_orm(skip)]
        pub attachments: Vec<ArticleAttachment>,

        /// 暂存附件数据行，由于转换为attachments时需要使用
        #[serde(skip)]
        #[sea_orm(skip)]
        pub attachment_rows: Vec<ArticleAttachmentRow>,

        pub published_at: Option<OffsetDateTime>,

        pub created_at: OffsetDateTime,

        pub updated_at: Option<OffsetDateTime>,
    }

    #[derive(Serialize, FromQueryResult)]
    pub struct ArticleExcerpt {
        pub id: Uuid,

        pub title: String,

        #[sea_orm(from_alias = "content")]
        pub excerpt: String,

        pub state: i16,

        /// none when state is draft
        pub category: Option<String>,

        pub published_at: Option<OffsetDateTime>,

        pub created_at: OffsetDateTime,

        pub updated_at: Option<OffsetDateTime>,
    }

    #[derive(Serialize, FromQueryResult)]
    pub struct ArticleAttachmentRow {
        pub id: Uuid,
        pub file_name: String,
    }

    #[derive(Serialize)]
    pub struct ArticleAttachment {
        pub id: Uuid,
        pub file_name: String,
        pub url: String,
    }
}
