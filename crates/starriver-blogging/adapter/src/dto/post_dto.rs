use sea_orm::FromQueryResult;
use starriver_blogging_application::dto::post_dto::res::PostExcerptDto;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(FromQueryResult)]
pub struct PostExcerptRow {
    pub id: Uuid,
    pub title: String,
    #[sea_orm(from_alias = "content")]
    pub excerpt: String,
    pub state: i16,
    pub category: String,
    pub published_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(FromQueryResult)]
pub struct PostDetailRow {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub state: i16,
    pub category_id: Uuid,
    pub category_name: String,
    pub published_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

//////////////////////////////////////////////////

impl From<PostExcerptRow> for PostExcerptDto {
    fn from(value: PostExcerptRow) -> Self {
        PostExcerptDto {
            id: value.id,
            title: value.title,
            excerpt: value.excerpt,
            state: value.state,
            category: value.category,
            published_at: value.published_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
