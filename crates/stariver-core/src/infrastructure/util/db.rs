use std::env;
use std::sync::OnceLock;

use sea_orm::{Database, DatabaseConnection};

static DB_CONN: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn db_conn() -> &'static DatabaseConnection {
    let db_url = env::var("DB_URL").expect("DB_URL environment variable not set");
    let conn = Database::connect(db_url)
        .await
        .expect("create a DatabaseConnection failed");
    DB_CONN.get_or_init(|| conn)
}
