mod config;

use axum::Router;

use mimalloc::MiMalloc;
use sea_orm::Database;
use starriver_blogging_adapter::port_in::{router as blogging_router, state::BloggingState};
use starriver_identity_adapter::port_in::router as identity_router;
use starriver_identity_adapter::port_in::state::IdentityState;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{error, info};

use crate::config::{
    authentication::{UsernamePasswordAuthenticator, build_authentication_layer},
    config_service::load_config,
    tracing::{init_tracing, tracing_span},
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    let app_cfg = load_config().unwrap_or_else(|e| panic!("failed to load config: {}", e));

    init_tracing(&app_cfg.logging);

    let addrs = (app_cfg.http_server.ip, app_cfg.http_server.port);

    let conn = Database::connect(app_cfg.database.url)
        .await
        .unwrap_or_else(|e| {
            error!(error = %e, "failed to connect to database");
            panic!("failed to connect to database: {}", e);
        });
    let auth = app_cfg.auth;
    let identity_state = IdentityState::new(conn.clone(), auth, &app_cfg.ctx_identity)
        .await
        .unwrap_or_else(|e| {
            error!(error = %e, "failed to create identity state");
            panic!("failed to create identity state: {}", e);
        });
    let blogging_state = BloggingState::new(conn.clone()).await.unwrap_or_else(|e| {
        error!(error = %e, "failed to create blogging state");
        panic!("failed to create blogging state: {}", e);
    });

    let user_service = identity_state.user_service.clone();
    let auth = identity_state.auth.clone();
    let middleware_service = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tracing_span)
                .on_request(DefaultOnRequest::default().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::default().level(tracing::Level::INFO))
                .on_failure(DefaultOnFailure::default().level(tracing::Level::INFO)),
        )
        .layer(build_authentication_layer(
            UsernamePasswordAuthenticator { user_service },
            auth,
        ));

    let router = Router::new()
        .merge(identity_router::create_router(identity_state))
        .merge(blogging_router::create_router(blogging_state))
        .layer(middleware_service);

    let listener = TcpListener::bind(addrs).await.unwrap_or_else(|e| {
        error!(error = %e, "can't bind to addr");
        std::process::exit(1);
    });

    let bound_addr = listener.local_addr().unwrap_or_else(|e| {
        error!(error = %e, "listener missing local addr");
        std::process::exit(1);
    });
    info!("Server listening on {}", bound_addr);
    if let Err(e) = axum::serve(listener, router).await {
        error!(error = %e, "server terminated with error");
    }
}
