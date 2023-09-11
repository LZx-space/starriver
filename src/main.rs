use std::env;
use std::io::{stdout, BufWriter};
use std::net::IpAddr;

use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::{middleware, web, App, HttpServer};
use ferris_says::say;
use sea_orm::DatabaseConnection;

use adapter::api::blog_api;

use crate::adapter::api::authentication_api;
use crate::adapter::repository::article_repository::ArticleRepositoryImpl;
use crate::application::blog_service::ArticleApplication;
use crate::infrastructure::security::authentication::web::actix::middleware::AuthenticateStateTransform;
use crate::infrastructure::util::db::db_conn;

mod adapter;
mod application;
mod domain;
mod infrastructure;

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
            .wrap(middleware::Logger::default())
            .wrap(middleware::ErrorHandlers::new())
            .wrap(
                actix_session::SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(&[0; 64]),
                )
                .cookie_secure(false)
                .build(),
            )
            .wrap(AuthenticateStateTransform {})
            .app_data(web::Data::new(app_state))
            .service(authentication_api::login_in)
            .service(authentication_api::validate_authenticated)
            .service(blog_api::page)
            .service(blog_api::find_one)
            .service(blog_api::insert)
            .service(blog_api::delete)
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

/// 应用的各个状态
///
pub struct AppState {
    article_application: ArticleApplication<ArticleRepositoryImpl>,
}

impl AppState {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        let article_application = ArticleApplication::new(ArticleRepositoryImpl { conn });
        AppState {
            article_application,
        }
    }
}
