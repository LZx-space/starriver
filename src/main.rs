use std::io::{stdout, BufWriter};

use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::{middleware, web, App, HttpServer};
use ferris_says::say;
use sea_orm::{Database, DatabaseConnection};

use adapter::api::blog_api;

use crate::adapter::api::authentication_api;
use crate::infrastructure::security::authentication::core::form::AuthenticateStateTransform;

mod adapter;
mod application;
mod domain;
mod infrastructure;

fn say_hello() {
    let out = "Hello, world!";
    let width = out.len();
    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    say_hello();
    const URL: &str = "postgresql://postgres:postgres@localhost:5432/stariver";
    let conn = Database::connect(URL).await.unwrap();
    let app_state = AppState { conn };
    HttpServer::new(move || {
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
            .app_data(web::Data::new(app_state.clone()))
            .service(
                actix_files::Files::new("/static", ".")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(authentication_api::login_in)
            .service(authentication_api::validate_authenticated)
            .service(blog_api::page)
            .service(blog_api::find_one)
            .service(blog_api::insert)
            .service(blog_api::delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}
