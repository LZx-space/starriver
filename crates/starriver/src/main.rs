use axum::Router;

use axum::routing::{get, post};
use mimalloc::MiMalloc;
use starriver_adapter::api::user_handler;
use starriver_adapter::api::{article_handler, category_handler};
use starriver_adapter::config::app_state::AppState;
use starriver_adapter::config::username_password_authenticator::UsernamePasswordAuthenticator;
use starriver_infrastructure::security::authentication::web::middleware::AuthenticationLayer;
use starriver_infrastructure::service::config_service::{FileLogging, load_config};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::level_filters::LevelFilter;
use tracing::{Level, error, info};
use tracing_subscriber::Layer;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    let config = load_config().unwrap_or_else(|e| panic!("failed to load config: {}", e));

    init_tracing(&config.file_logging);

    let addrs = (config.http_server.ip.clone(), config.http_server.port);
    let app_state = AppState::new(config).await.unwrap_or_else(|e| {
        error!("failed to create app state: {}", e);
        std::process::exit(1);
    });

    let user_service = app_state.user_application.clone();
    let auth_cfg = app_state.auth_cfg.clone();

    let middleware_service = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tracing_span)
                .on_request(DefaultOnRequest::default().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::default().level(tracing::Level::INFO))
                .on_failure(DefaultOnFailure::default().level(tracing::Level::INFO)),
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

/////////////////////////////////////////////////////////////////////////////////////////////

/// Initializes the tracing subsystem using the provided logging configuration.
fn init_tracing(cfg: &FileLogging) {
    use std::io::IsTerminal;
    use tracing_appender::rolling::{RollingFileAppender, Rotation};
    use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    let level = filter
        .max_level_hint()
        .unwrap_or(LevelFilter::INFO)
        .into_level()
        .unwrap_or(Level::INFO);
    if cfg.file_enabled {
        // ---------- 仅文件日志 ----------
        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix(&cfg.file_name_prefix)
            .max_log_files(cfg.max_files)
            .build(&cfg.file_directory)
            .expect("failed to create rolling file appender");

        let file_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_writer(file_appender)
            .with_span_list(false)
            .with_target(false)
            .with_file(false)
            .with_line_number(false)
            .with_filter(filter);

        tracing_subscriber::registry().with(file_layer).init();

        println!(
            "tracing initialized log_level={} log_dir={}",
            level, cfg.file_directory
        );
    } else {
        // ---------- 仅控制台日志 ----------
        let console_layer = tracing_subscriber::fmt::layer()
            .with_ansi(std::io::stdout().is_terminal())
            .with_file(true)
            .with_line_number(true)
            .with_filter(filter);
        tracing_subscriber::registry().with(console_layer).init();
        info!(log_level = %level, "tracing initialized");
    }
}

/// custom tracing span for HTTP requests.
fn tracing_span(request: &axum::extract::Request) -> tracing::Span {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|hv| hv.to_str().ok())
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
