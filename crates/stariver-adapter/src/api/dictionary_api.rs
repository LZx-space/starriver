use actix_web::web::Json;
use actix_web::{post, web, Responder};

use stariver_core::infrastructure::service::dictionary::dictionary_service::{
    DataType, Dictionary, DictionaryEntry,
};

use crate::state::app_state::AppState;

#[post("/dictionary-entries")]
pub async fn add_dictionary_entry(state: web::Data<AppState>) -> impl Responder {
    let dictionary = Dictionary::new(state.conn);
    let entry =
        DictionaryEntry::new("66".to_string(), DataType::I8, "测试".to_string()).expect("123");
    match dictionary.insert(entry).await {
        None => Json("OK".to_string()),
        Some(e) => Json(e.to_string()),
    }
}
