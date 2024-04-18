use std::env;
use std::io::{stdout, BufWriter};
use std::net::IpAddr;

use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::{middleware, web, App, HttpServer};
use ferris_says::say;

use stariver_adapter::api::authentication_api;
use stariver_adapter::api::blog_api;
use stariver_adapter::api::dictionary_api;
use stariver_adapter::state::app_state::AppState;
use stariver_core::infrastructure::security::authentication::web::actix::middleware::AuthenticationTransform;
use stariver_core::infrastructure::util::db::db_conn;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    say_hello();
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let conn = db_conn().await;
    let addrs = http_server_bind_addrs();
    HttpServer::new(move || {
        let app_state = AppState::new(conn);
        App::new()
            .wrap(AuthenticationTransform {})
            .wrap(
                actix_session::SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(&[0; 64]),
                )
                .cookie_secure(false)
                .build(),
            )
            .wrap(middleware::Logger::default())
            .wrap(middleware::ErrorHandlers::new())
            .app_data(web::Data::new(app_state))
            .service(authentication_api::validate_authenticated)
            .service(blog_api::page)
            .service(blog_api::find_one)
            .service(blog_api::insert)
            .service(blog_api::delete)
            .service(dictionary_api::list_dictionary_entry)
            .service(dictionary_api::add_dictionary_entry)
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
