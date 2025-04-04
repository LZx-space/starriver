use actix_web::http::StatusCode;
use actix_web::web::{Json, Path, Query};
use actix_web::{Responder, delete, get, post, put, web};
use uuid::Uuid;

use crate::assembler::blog::{cmd_2_new_entity, cmd_2_update_entity};
use crate::config::app_state::AppState;
use crate::model::blog::{BlogCmd, BlogVo};
use starriver_infrastructure::model::page::PageQuery;

#[get("/blogs")]
pub async fn page(state: web::Data<AppState>, params: Query<PageQuery>) -> impl Responder {
    let page_query = params.into_inner();
    state
        .blog_application
        .page(page_query)
        .await
        .map(|e| Json(e))
}

#[get("/blogs/{id}")]
pub async fn find_one(state: web::Data<AppState>, id: Path<Uuid>) -> impl Responder {
    state
        .blog_application
        .find_by_id(id.into_inner())
        .await
        .map(|e| {
            e.map(|a| BlogVo {
                title: a.title,
                body: a.body,
                state: a.state.to_string(),
            })
        })
        .map(|e| Json(e))
}

#[post("/blogs")]
pub async fn insert(state: web::Data<AppState>, cmd: Json<BlogCmd>) -> impl Responder {
    let cmd = cmd.into_inner();
    let blog = cmd_2_new_entity(cmd, "LZx".to_string());
    state.blog_application.add(blog).await.map(|e| Json(e))
}

#[put("/blogs/{id}")]
pub async fn update(
    state: web::Data<AppState>,
    id: Path<Uuid>,
    cmd: Json<BlogCmd>,
) -> impl Responder {
    let cmd = cmd.into_inner();
    let id = id.into_inner();
    let to_update = state.blog_application.find_by_id(id).await.expect("");
    if to_update.is_none() {
        ()
    }
    let blog = cmd_2_update_entity(cmd, to_update.unwrap());
    state.blog_application.update(blog).await.map(|e| Json(e))
}

#[delete("/blogs/{id}")]
pub async fn delete(state: web::Data<AppState>, id: Path<Uuid>) -> impl Responder {
    state
        .blog_application
        .delete_by_id(id.into_inner())
        .await
        .expect("TODO: panic message");
    (Json("1"), StatusCode::UNAUTHORIZED)
}
