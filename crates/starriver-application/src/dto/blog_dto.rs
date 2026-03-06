use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BlogCmd {
    pub title: String,
    pub body: String,
}

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
