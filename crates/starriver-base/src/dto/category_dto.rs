pub mod req {
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
    pub struct CreateOrUpdateCategoryCmd {
        #[validate(length(min = 1, max = 10))]
        pub name: String,
    }
}

pub mod res {
    use sea_orm::FromQueryResult;
    use serde::Serialize;
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[derive(Serialize, FromQueryResult)]
    pub struct CategoryDetail {
        pub id: Uuid,
        pub name: String,
        pub created_at: OffsetDateTime,
        pub updated_at: Option<OffsetDateTime>,
    }
}
