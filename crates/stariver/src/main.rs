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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    say_hello();
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
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
            .wrap(middleware::Logger::default())
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
