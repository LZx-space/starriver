use sea_orm::DatabaseConnection;

/// 应用的各个状态
#[derive(Clone)]
pub struct BloggingState {
    pub conn: DatabaseConnection,
}

impl BloggingState {
    pub async fn new(conn: DatabaseConnection) -> Result<Self, String> {
        Ok(BloggingState { conn })
    }
}
