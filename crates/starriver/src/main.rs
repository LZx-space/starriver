use axum::Router;
use axum::error_handling::HandleErrorLayer;

use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use ferris_says::say;
use mimalloc::MiMalloc;
use starriver_adapter::api::blog;
use starriver_adapter::api::dictionary;
use starriver_adapter::api::{authentication, user};
use starriver_adapter::config::app_state::AppState;
use starriver_adapter::config::user_principal::{UserAuthenticator, UserRepositoryImpl};
use starriver_adapter::config::username_flow::UsernameFlow;
use starriver_infrastructure::error::error::AppError;
use starriver_infrastructure::security::authentication::web::axum::middleware::AuthenticationLayer;
use starriver_infrastructure::util::db::db_conn;
use std::env;
use std::io::{BufWriter, stdout};
use std::net::IpAddr;

use tower::ServiceBuilder;

use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    say_hello();
    dotenvy::dotenv().expect(".env file not found");

    let file_appender = tracing_appender::rolling::daily("./log", "starriver.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = layer()
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_filter(LevelFilter::INFO);

    let console_layer = layer()
        .with_writer(stdout)
        .with_timer(LocalTime::rfc_3339())
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();

    let conn = db_conn().await;
    let addrs = http_server_bind_addrs();
    let handle_error_layer = HandleErrorLayer::new(handle_error);
    let authentication_layer = AuthenticationLayer::new(
        UserAuthenticator::new(UserRepositoryImpl::new(conn)),
        UsernameFlow {},
    );

    let service_builder = ServiceBuilder::new()
        .layer(handle_error_layer)
        .layer(authentication_layer);
    let router = Router::new()
        .route("/session/user", get(authentication::validate_authenticated))
        .route("/users", post(user::insert))
        .route("/blogs", get(blog::page).post(blog::insert))
        .route(
            "/blogs/{id}",
            get(blog::find_one).put(blog::update).delete(blog::delete),
        )
        .route(
            "/dictionary-entries",
            get(dictionary::list_dictionary_entry).post(dictionary::add_dictionary_entry),
        )
        .layer(service_builder)
        .with_state(AppState::new(conn));
    let listener = TcpListener::bind(&addrs)
        .await
        .expect("Can't bind to address");
    axum::serve(listener, router).await.unwrap();
}

fn say_hello() {
    let out = "Hello, World!";
    let width = out.len();
    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap()
}

async fn handle_error(error: AppError) -> Response {
    // todo 记录错误
    eprintln!("异常：{}", error);
    error.into_response()
}

fn http_server_bind_addrs() -> (IpAddr, u16) {
    let http_server_ip =
        env::var("HTTP_SERVER_IP").expect("HTTP_SERVER_IP environment variable not set");
    let http_server_port =
        env::var("HTTP_SERVER_PORT").expect("HTTP_SERVER_PORT environment variable not set");
    (
        http_server_ip.parse().unwrap(),
        http_server_port.parse().unwrap(),
    )
}
