pub mod req {
    use bytes::Bytes;
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
    pub struct BlogCmd {
        #[validate(length(min = 1, max = 30))]
        pub title: String,
        #[validate(length(min = 1, max = 50000))]
        pub content: String,
        #[validate(length(min = 0, max = 10))]
        pub attachment_ids: Vec<uuid::Uuid>,
        /// 是否发布
        pub publish: bool,
    }

    pub struct BlogAttachmentCmd {
        pub extension: String,
        pub data: Bytes,
    }
}

pub mod res {
    use sea_orm::FromQueryResult;
    use serde::Serialize;
    use starriver_domain::blog::entity::Blog;
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct BlogDetail {
        pub id: Uuid,

        pub title: String,

        pub content: String,

        pub state: String,
    }

    #[derive(Serialize, FromQueryResult)]
    pub struct BlogExcerpt {
        pub id: Uuid,

        pub title: String,

        #[sea_orm(from_alias = "content")]
        pub excerpt: String,

        pub state: i16,

        pub create_at: OffsetDateTime,
    }

    //////////////////////////////////////////
    impl From<Blog> for BlogDetail {
        fn from(value: Blog) -> Self {
            let (id, title, content, state, _, _, _, _) = value.dissolve();
            Self {
                id,
                title: title.to_string(),
                content: content.to_string(),
                state: state.to_string(),
            }
        }
    }
}
