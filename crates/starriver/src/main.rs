use axum::Router;

use axum::routing::{get, post};
use ferris_says::say;
use mimalloc::MiMalloc;
use starriver_adapter::api::blog_handler;
use starriver_adapter::api::dictionary_handler;
use starriver_adapter::api::{authentication_handler, user_handler};
use starriver_adapter::config::app_state::AppState;
use starriver_adapter::config::username_password_authentictor::{
    TokioTimingAttackProtection, UsernamePasswordAuthenticator,
};
use starriver_adapter::config::username_password_flow::UsernamePasswordFlow;
use starriver_infrastructure::security::authentication::web::middleware::AuthenticationLayer;
use starriver_infrastructure::util::db::db_conn;
use std::env;
use std::io::{BufWriter, stdout};
use std::net::IpAddr;
use std::time::Duration;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use tower::ServiceBuilder;

use tokio::net::TcpListener;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    say_hello();
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().expect(".env file not found");

    let conn = db_conn().await;
    let addrs = http_server_bind_addrs();
    let app_state = AppState::new(conn);
    let user_service = app_state.user_application.clone();

    let serve_dir = ServeDir::new("static").fallback(ServeDir::new("static/index.html"));
    let middleware_service = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(AuthenticationLayer::new(
            UsernamePasswordAuthenticator { user_service },
            UsernamePasswordFlow {},
            TokioTimingAttackProtection {
                delay: Duration::from_millis(500),
            },
        ));
    let router = Router::new()
        .route(
            "/session/user",
            get(authentication_handler::authenticated_user),
        )
        .route("/users", post(user_handler::insert))
        .route("/blogs", get(blog_handler::page).post(blog_handler::insert))
        .route(
            "/blogs/{id}",
            get(blog_handler::find_one)
                .put(blog_handler::update)
                .delete(blog_handler::delete),
        )
        .route(
            "/dictionary-entries",
            get(dictionary_handler::list_dictionary_entry)
                .post(dictionary_handler::add_dictionary_entry),
        )
        .nest_service("/static", serve_dir)
        .with_state(app_state)
        .layer(middleware_service);
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
