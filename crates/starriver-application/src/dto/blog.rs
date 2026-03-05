use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct BlogCmd {
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct BlogVo {
    pub title: String,

    pub body: String,

    pub state: String,
}
