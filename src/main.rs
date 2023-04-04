use std::io::{stdout, BufWriter};

use actix_web::{middleware, web, App, HttpServer};
use ferris_says::say;
use sea_orm::{Database, DatabaseConnection};

use adapter::api::blog_api;

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
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    say_hello();
    const URL: &str =
        "postgresql://postgres:123456@localhost:5432/stariver?serverTimezone=Asia/Shanghai\
    &autoReconnect=false&useUnicode=true&characterEncoding=UTF-8&characterSetResults=UTF-8&\
    zeroDateTimeBehavior=convertToNull&useSSL=false";
    let conn = Database::connect(URL).await.unwrap();
    let app_state = AppState { conn };
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(app_state.clone()))
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
