use tracing::{Level, error, info, level_filters::LevelFilter};
use tracing_subscriber::Layer;

use crate::config::config_service::Logging;

/// * Initializes the tracing subsystem using the provided logging configuration and env variable `RUST_LOG`.
/// * default is INFO.
pub fn init_tracing(cfg: &Logging) {
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
        // ---------- only file logging ----------
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
        // ---------- only console logging ----------
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
pub fn tracing_span(request: &axum::extract::Request) -> tracing::Span {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|hv| hv.to_str().ok())
        .unwrap_or_else(|| {
            error!("failed to get x-request-id from headers");
            "unknow"
        });
    tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        uri = %request.uri(),
        status_code = tracing::field::Empty,
        latency_ms = tracing::field::Empty,
    )
}
