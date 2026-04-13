use axum::Router;

use axum::routing::{get, post};
use ferris_says::say;
use mimalloc::MiMalloc;
use starriver_adapter::api::article_handler;
use starriver_adapter::api::user_handler;
use starriver_adapter::config::app_state::AppState;
use starriver_adapter::config::username_password_authenticator::UsernamePasswordAuthenticator;
use starriver_infrastructure::security::authentication::web::middleware::AuthenticationLayer;
use starriver_infrastructure::service::config_service::load_config;
use std::io::{BufWriter, stdout};
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

use tower::ServiceBuilder;

use tokio::net::TcpListener;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    say_hello();
    tracing_subscriber::fmt::init();
    let config = load_config().expect("failed to load config");

    let addrs = (config.http_server.ip.clone(), config.http_server.port);
    let app_state = AppState::new(config)
        .await
        .expect("failed to create app state");

    let user_service = app_state.user_application.clone();
    let middleware_service = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(AuthenticationLayer::with_authenticator(
            UsernamePasswordAuthenticator { user_service },
        ));
    let router = Router::new()
        .route("/users/me", get(user_handler::me))
        .route("/users", post(user_handler::register_inactive_user))
        .route("/email-verifications", post(user_handler::verify_email))
        .route(
            "/articles",
            get(article_handler::page).post(article_handler::insert_empty_draft),
        )
        .route(
            "/articles/{id}",
            get(article_handler::find_one)
                .put(article_handler::update)
                .delete(article_handler::delete),
        )
        .route(
            "/articles/{id}/attachments",
            post(article_handler::upload_attachment),
        )
        .with_state(app_state)
        .layer(middleware_service);
    let listener = TcpListener::bind(addrs)
        .await
        .expect("Can't bind to address");
    axum::serve(listener, router)
        .await
        .expect("Can't serve the service");
}

fn say_hello() {
    let out = "Hello, World!";
    let width = out.len();
    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).expect("Can't write to stdout")
}
