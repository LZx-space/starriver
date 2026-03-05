use crate::config::username_password_authentictor::User;
use axum::Json;
use axum::response::IntoResponse;
use starriver_infrastructure::security::authentication::core::principal::Principal;

#[axum::debug_handler]
pub async fn validate_authenticated(user: User) -> impl IntoResponse {
    let username = user.id().as_str();
    Json(String::from(username))
}
