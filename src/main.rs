use std::env;
use std::io::{stdout, BufWriter};

use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::{middleware, web, App, HttpServer};
use async_static::async_static;
use ferris_says::say;
use sea_orm::{Database, DatabaseConnection};

use adapter::api::blog_api;

use crate::adapter::api::authentication_api;
use crate::adapter::repository::article_repository::ArticleRepositoryImpl;
use crate::application::article_service::ArticleApplication;
use crate::infrastructure::security::authentication::web::actix::middleware::AuthenticateStateTransform;

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

    let conn = CONN.await;
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn say_hello() {
    let out = "Hello, world!";
    let width = out.len();
    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap()
}

async_static! {
    static ref CONN: DatabaseConnection = init_db_conn().await;
}

async fn init_db_conn() -> DatabaseConnection {
    let db_url = env::var("DB_URL").expect("DB_URL environment variable not set");
    Database::connect(db_url)
        .await
        .expect("create a DatabaseConnection failed")
}

/// 应用的各个状态
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
