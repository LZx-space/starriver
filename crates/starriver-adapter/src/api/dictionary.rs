use crate::config::app_state::AppState;
use axum::Json;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use starriver_infrastructure::error::error::{AppError, Cause};
use starriver_infrastructure::model::page::PageQuery;
use starriver_infrastructure::service::dictionary::dictionary_service::{
    DataType, DictionaryEntry,
};

pub async fn list_dictionary_entry(
    state: State<AppState>,
    query: Query<PageQuery>,
) -> impl IntoResponse {
    let page = state.dictionary.page(query.0).await;
    page.map(|u| Json(u))
        .map_err(|e| AppError::new(Cause::DbError, e.to_string()))
}

pub async fn add_dictionary_entry(state: State<AppState>) -> impl IntoResponse {
    let entry =
        DictionaryEntry::new("66".to_string(), DataType::I8, "测试".to_string()).expect("123");
    match state.dictionary.insert(entry).await {
        None => Json("OK".to_string()),
        Some(e) => Json(e.to_string()),
    }
}
