use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ArticlePageItem {
    pub id: Uuid,

    pub title: String,

    pub release_date: String,

    pub tags: Vec<String>,
}
