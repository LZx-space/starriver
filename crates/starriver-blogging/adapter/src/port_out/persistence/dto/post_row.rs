use sea_orm::FromQueryResult;
use starriver_blogging_application::dto::post_dto::res::PostExcerptDto;
use starriver_blogging_domain::post::value_object::PostState;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::port_out::persistence::po::post_po::PostStatePo;

#[derive(FromQueryResult)]
pub struct PostExcerptRow {
    pub id: Uuid,
    pub title: String,
    #[sea_orm(from_alias = "content")]
    pub excerpt: String,
    pub state: PostStatePo,
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
    pub state: PostStatePo,
    pub category_id: Uuid,
    pub category_name: String,
    pub published_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(FromQueryResult)]
pub struct PostSearchRow {
    pub id: Uuid,
    pub title: String,
    pub published_at: OffsetDateTime, // 未发布的才有可能为空
    pub category: String,
    pub snippet: String,
    pub rank: f32,
}

//////////////////////////////////////////////////

impl From<PostExcerptRow> for PostExcerptDto {
    fn from(value: PostExcerptRow) -> Self {
        PostExcerptDto {
            id: value.id,
            title: value.title,
            excerpt: value.excerpt,
            state: PostState::from(value.state).to_string(),
            category: value.category,
            published_at: value.published_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
