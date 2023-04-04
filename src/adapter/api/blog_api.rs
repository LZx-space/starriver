use actix_web::http::StatusCode;
use actix_web::web::{Json, Path, Query};
use actix_web::{delete, get, post, put, web, Responder};
use sea_orm::prelude::Uuid;
use sea_orm::DatabaseConnection;

use crate::adapter::api::blog_model::{ArticleCmd, ArticleVo};
use crate::adapter::assembler::article_assembler::{cmd_2_new_entity, cmd_2_update_entity};
use crate::adapter::repository::article_repository::ArticleRepositoryImpl;
use crate::application::article_service::ArticleApplication;
use crate::infrastructure::page::PageQuery;
use crate::AppState;

#[get("/blogs")]
pub async fn page(state: web::Data<AppState>, params: Query<PageQuery>) -> impl Responder {
    let page_query = params.into_inner();
    let result = article_application(&state.conn).page(page_query).await;
    Json(result.unwrap())
}

#[get("/blogs/{id}")]
pub async fn find_one(state: web::Data<AppState>, id: Path<Uuid>) -> impl Responder {
    let repository_impl = ArticleRepositoryImpl { conn: &state.conn };
    let result = article_application(&state.conn)
        .find_one(id.into_inner())
        .await;
    let option = result.unwrap();
    let article = option.expect("记录不存在");
    let vo = ArticleVo {
        title: article.title,
        body: article.body,
        tags: vec![],
    };
    Json(vo)
}

#[post("/blogs")]
pub async fn insert(state: web::Data<AppState>, cmd: Json<ArticleCmd>) -> impl Responder {
    let cmd = cmd.into_inner();
    let article = cmd_2_new_entity(cmd, "LZx".to_string());
    let result = article_application(&state.conn).add(article).await;
    Json(result.unwrap())
}

#[put("/blogs/{id}")]
pub async fn update(
    state: web::Data<AppState>,
    id: Path<Uuid>,
    cmd: Json<ArticleCmd>,
) -> impl Responder {
    let cmd = cmd.into_inner();
    let id = id.into_inner();
    let article = cmd_2_update_entity(cmd, id, "LZx".to_string());
    let result = article_application(&state.conn).update(article).await;
    Json(result.unwrap())
}

#[delete("/blogs/{id}")]
pub async fn delete(state: web::Data<AppState>, id: Path<Uuid>) -> impl Responder {
    article_application(&state.conn)
        .delete(id.into_inner())
        .await
        .expect("TODO: panic message");
    (Json("1"), StatusCode::UNAUTHORIZED)
}

fn article_application(conn: &DatabaseConnection) -> ArticleApplication<ArticleRepositoryImpl> {
    let repository_impl = ArticleRepositoryImpl { conn };
    ArticleApplication::new(repository_impl)
}
