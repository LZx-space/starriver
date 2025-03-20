use actix_web::web::{Json, Query};
use actix_web::{Responder, get, post, web};

use crate::app_state::AppState;
use stariver_infrastructure::model::err::CodedErr;
use stariver_infrastructure::model::page::PageQuery;
use stariver_infrastructure::service::dictionary::dictionary_service::{DataType, DictionaryEntry};

#[get("/dictionary-entries")]
pub async fn list_dictionary_entry(
    state: web::Data<AppState>,
    query: Query<PageQuery>,
) -> impl Responder {
    let page = state.dictionary.page(query.into_inner()).await;
    page.map(|u| Json(u))
        .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
}

#[post("/dictionary-entries")]
pub async fn add_dictionary_entry(state: web::Data<AppState>) -> impl Responder {
    let entry =
        DictionaryEntry::new("66".to_string(), DataType::I8, "测试".to_string()).expect("123");
    match state.dictionary.insert(entry).await {
        None => Json("OK".to_string()),
        Some(e) => Json(e.to_string()),
    }
}
