use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ArticleCmd {
    pub title: String,
    pub body: String,
    pub tags: Vec<i64>,
}

#[derive(Serialize)]
pub struct ArticleVo {
    pub title: String,

    pub body: String,

    pub tags: Vec<TagVo>,
}

#[derive(Serialize)]
pub struct TagVo {
    pub name: String,
}

#[derive(Serialize)]
pub struct ArticleSummary {
    pub id: i64,

    pub title: String,

    pub release_date: String,

    pub tags: Vec<String>,
}
