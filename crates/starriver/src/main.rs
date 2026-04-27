use axum::Router;

use axum::routing::{get, post};
use mimalloc::MiMalloc;
use starriver_adapter::api::user_handler;
use starriver_adapter::api::{article_handler, category_handler};
use starriver_adapter::config::app_state::AppState;
use starriver_adapter::config::username_password_authenticator::UsernamePasswordAuthenticator;
use starriver_infrastructure::security::authentication::web::middleware::AuthenticationLayer;
use starriver_infrastructure::service::config_service::{Logging, load_config};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    let config = load_config().expect("failed to load config");

    init_tracing(&config.logging);

    info!(
        log_level = %config.logging.level,
        log_dir = %config.logging.directory,
        "tracing initialized"
    );

    let addrs = (config.http_server.ip.clone(), config.http_server.port);
    let app_state = AppState::new(config)
        .await
        .expect("failed to create app state");

    let user_service = app_state.user_application.clone();
    let auth_cfg = app_state.auth_cfg.clone();

    let middleware_service = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tracing_span)
                .on_request(tracing_on_request)
                .on_response(tracing_on_response)
                .on_failure(tracing_on_failure),
        )
        .layer(AuthenticationLayer::with_authenticator(
            UsernamePasswordAuthenticator { user_service },
            auth_cfg,
        ));
    let router = Router::new()
        .route("/users/me", get(user_handler::me))
        .route("/users", post(user_handler::register_inactive_user))
        .route("/email-verifications", post(user_handler::verify_email))
        .route(
            "/articles",
            get(article_handler::paginate).post(article_handler::create_draft),
        )
        .route(
            "/articles/{id}",
            get(article_handler::show)
                .put(article_handler::update)
                .delete(article_handler::delete),
        )
        .route(
            "/articles/{id}/attachments",
            post(article_handler::upload_attachment),
        )
        .route(
            "/categories",
            get(category_handler::list_all).post(category_handler::create),
        )
        .route(
            "/categories/{id}",
            get(category_handler::show)
                .put(category_handler::update)
                .delete(category_handler::delete),
        )
        .with_state(app_state)
        .layer(middleware_service);
    let listener = TcpListener::bind(addrs)
        .await
        .expect("Can't bind to address");

    let bound_addr = listener.local_addr().expect("missing local addr");
    info!("Server listening on {}", bound_addr);
    axum::serve(listener, router)
        .await
        .expect("Can't serve the service");
}

/////////////////////////////////////////////////////////////////////////////////////////////

/// Initializes the tracing subsystem using the provided logging configuration.
fn init_tracing(cfg: &Logging) {
    use std::io::IsTerminal;
    use tracing_appender::rolling::{RollingFileAppender, Rotation};
    use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

    // ---- File log: JSON format (debug level, always on) ----
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix(&cfg.file_name_prefix)
        .max_log_files(cfg.max_files)
        .build(&cfg.directory)
        .expect("failed to create rolling file appender");

    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(file_appender)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_level(true)
        .with_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")));

    // ---- Console log: formatted text (configurable level) ----
    let console_layer = tracing_subscriber::fmt::layer()
        .with_ansi(std::io::stdout().is_terminal())
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        .with_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&cfg.level)),
        );

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();
}

// ---------------------------------------------------------------------------
// tracing custom span and log helpers
// ---------------------------------------------------------------------------

fn tracing_span(request: &axum::extract::Request) -> tracing::Span {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        uri = %request.uri(),
        status_code = tracing::field::Empty,
        latency_ms = tracing::field::Empty,
    )
}

fn tracing_on_request(request: &axum::extract::Request, _span: &tracing::Span) {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    info!(
        request_id = %request_id,
        method = %request.method(),
        uri = %request.uri(),
        "incoming request"
    );
}

fn tracing_on_response(
    response: &axum::response::Response,
    latency: std::time::Duration,
    span: &tracing::Span,
) {
    let latency_ms = latency.as_secs_f64() * 1000.0;
    span.record("status_code", response.status().as_u16());
    span.record("latency_ms", latency_ms);
    info!(
        status = %response.status().as_u16(),
        latency_ms = %latency_ms,
        "request completed"
    );
}

fn tracing_on_failure(
    _failure_class: ServerErrorsFailureClass,
    latency: std::time::Duration,
    _span: &tracing::Span,
) {
    let latency_ms = latency.as_secs_f64() * 1000.0;
    warn!(
        latency_ms = %latency_ms,
        "request failed"
    );
}
