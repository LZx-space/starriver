use std::env;
use std::io::{stdout, BufWriter};
use std::net::IpAddr;

use actix_web::{middleware, web, App, HttpServer};
use ferris_says::say;
use stariver_adapter::api::blog;
use stariver_adapter::api::dictionary;
use stariver_adapter::api::{authentication, user};
use stariver_core::infrastructure::security::authentication::user_principal::{
    UserAuthenticator, UserRepositoryImpl,
};
use stariver_core::infrastructure::security::authentication::web::actix::flow::username_flow::UsernameFlow;
use stariver_core::infrastructure::security::authentication::web::actix::middleware::AuthenticationTransform;
use stariver_core::infrastructure::util::db::db_conn;
use stariver_core::infrastructure::web::app_state::AppState;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    say_hello();
    dotenvy::dotenv().expect(".env file not found");

    let file_appender = tracing_appender::rolling::daily("./log", "stariver.log");
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

    HttpServer::new(move || {
        let app_state = AppState::new(conn);
        App::new()
            .wrap(AuthenticationTransform::new(
                UserAuthenticator::new(UserRepositoryImpl::new(conn)),
                UsernameFlow {},
            ))
            .wrap(middleware::ErrorHandlers::new())
            .app_data(web::Data::new(app_state))
            .service(authentication::validate_authenticated)
            .service(user::insert)
            .service(blog::page)
            .service(blog::find_one)
            .service(blog::insert)
            .service(blog::update)
            .service(blog::delete)
            .service(dictionary::list_dictionary_entry)
            .service(dictionary::add_dictionary_entry)
    })
    .bind(addrs)?
    .run()
    .await
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
