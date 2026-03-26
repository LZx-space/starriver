pub mod req {
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
    pub struct BlogCmd {
        #[validate(length(min = 1, max = 30))]
        pub title: String,
        #[validate(length(min = 1, max = 50000))]
        pub body: String,
    }
}

pub mod res {
    use sea_orm::FromQueryResult;
    use serde::Serialize;
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct BlogDetail {
        pub title: String,

        pub body: String,

        pub state: String,
    }

    #[derive(Serialize, FromQueryResult)]
    pub struct BlogSummary {
        pub id: Uuid,

        pub title: String,

        #[sea_orm(from_alias = "body")]
        pub summary: String,

        pub create_at: OffsetDateTime,
    }
}
