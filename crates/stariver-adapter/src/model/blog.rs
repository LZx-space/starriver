use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ArticleCmd {
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct ArticleVo {
    pub title: String,

    pub body: String,

    pub state: String,
}
