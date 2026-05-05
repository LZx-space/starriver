mod config;

use axum::Router;

use mimalloc::MiMalloc;
use sea_orm::Database;
use starriver_identity_adapter::port_in::router::create_router;
use starriver_identity_adapter::port_in::state::IdentityState;
use tokio::net::TcpListener;
use tracing::{error, info};

use crate::config::config_service::load_config;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    let app_cfg = load_config().unwrap_or_else(|e| panic!("failed to load config: {}", e));

    let addrs = (app_cfg.http_server.ip, app_cfg.http_server.port);
    let conn = Database::connect(app_cfg.database.url)
        .await
        .unwrap_or_else(|e| panic!("failed to connect to database: {}", e));

    let identity_state = IdentityState::new(conn, &app_cfg.ctx_identity_cfg)
        .await
        .unwrap_or_else(|e| panic!("failed to create identity state: {}", e));

    let router = Router::new().merge(create_router(identity_state));
    let listener = TcpListener::bind(addrs).await.unwrap_or_else(|e| {
        error!("Can't bind to address: {}", e);
        std::process::exit(1);
    });

    let bound_addr = listener.local_addr().unwrap_or_else(|e| {
        error!("listener missing local addr: {}", e);
        std::process::exit(1);
    });
    info!("Server listening on {}", bound_addr);
    if let Err(e) = axum::serve(listener, router).await {
        error!(error = %e, "server terminated with error");
    }
}
