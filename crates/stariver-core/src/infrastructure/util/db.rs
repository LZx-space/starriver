use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::OnceLock;

static DB_CONN: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn db_conn() -> &'static DatabaseConnection {
    if let Some(conn) = DB_CONN.get() {
        return conn;
    }
    let db_url = env::var("DB_URL").expect("DB_URL environment variable not set");
    let conn = Database::connect(db_url)
        .await
        .expect("create a DatabaseConnection failed");
    DB_CONN.get_or_init(|| conn)
}
