mod config;

use std::sync::Arc;

use axum::Router;
use mimalloc::MiMalloc;
use sea_orm::Database;
use starriver_blogging_adapter::port_in::{router as blogging_router, state::BloggingState};
use starriver_identity_adapter::port_in::router as identity_router;
use starriver_identity_adapter::port_in::state::IdentityState;
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    csrf::CsrfLayer,
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
            error!(error = %e, "connect to database");
            panic!("failed to connect to database: {}", e);
        });

    // configure each bounded ctx state
    let auth = Arc::new(app_cfg.auth);
    let uploads = Arc::new(app_cfg.uploads);
    let identity_state = IdentityState::new(conn.clone(), auth.clone(), &app_cfg.ctx_identity)
        .await
        .unwrap_or_else(|e| {
            error!(error = %e, "create identity state");
            panic!("failed to create identity state: {}", e);
        });
    let blogging_state = BloggingState::new(
        conn.clone(),
        auth.clone(),
        uploads.clone(),
        &app_cfg.ctx_blogging,
    )
    .await
    .unwrap_or_else(|e| {
        error!(error = %e, "create blogging state");
        panic!("failed to create blogging state: {}", e);
    });

    // configure middleware
    let csrf_layer = if !app_cfg.csrf.enabled {
        None
    } else {
        let mut csrf_layer = CsrfLayer::new();
        for origin in &app_cfg.csrf.trusted_origins {
            info!("adding csrf trusted origin: {}", origin);
            csrf_layer = csrf_layer.add_trusted_origin(origin).unwrap_or_else(|e| {
                error!(error = %e, "configure csrf layer");
                panic!("failed to config csrf layer with origin: {}", e);
            });
        }
        Some(csrf_layer)
    };

    let middleware_service = ServiceBuilder::new()
        .layer(CompressionLayer::new().gzip(true).br(true))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tracing_span)
                .on_request(DefaultOnRequest::default().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::default().level(tracing::Level::INFO))
                .on_failure(DefaultOnFailure::default().level(tracing::Level::INFO)),
        )
        .option_layer(csrf_layer)
        .layer(build_authentication_layer(
            UsernamePasswordAuthenticator {
                user_service: identity_state.user_service.clone(),
                cfg: auth.clone(),
            },
            auth,
        ));

    // configure router
    let router = Router::new()
        .merge(identity_router::create_router(identity_state))
        .merge(blogging_router::create_router(blogging_state))
        .layer(middleware_service);

    // configure server
    let listener = TcpListener::bind(addrs).await.unwrap_or_else(|e| {
        error!(error = %e, "listener bind addr");
        panic!("failed to bind local address");
    });

    let bound_addr = listener.local_addr().unwrap_or_else(|e| {
        error!(error = %e, "listener local addr");
        panic!("failed to get local address");
    });
    info!(addr = %bound_addr, "server listening");
    if let Err(e) = axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal(async {
            info!("graceful shutdown completed");
        }))
        .await
    {
        error!(error = %e, "server start failed");
    }
}

async fn shutdown_signal(handler: impl Future<Output = ()>) {
    // listen for ctrl+c and terminate signals
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler");
    };
    let terminate = cfg_select! {
        unix => {
            async {
                signal::unix::signal(signal::unix::SignalKind::terminate())
                    .expect("failed to install signal handler")
                    .recv()
                    .await;
            }
        }
        _ => { std::future::pending::<()>() }
    };
    tokio::select! {
        _ = ctrl_c => {
            info!("ctrl+c received, starting graceful shutdown");
            handler.await
        }
        _ = terminate => {
            info!("terminate received, starting graceful shutdown");
            handler.await
        }
    };
}
