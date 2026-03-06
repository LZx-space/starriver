use crate::config::username_password_authentictor::User;
use axum::Json;
use axum::response::IntoResponse;
use starriver_infrastructure::security::authentication::core::principal::Principal;

pub async fn authenticated_user(user: User) -> impl IntoResponse {
    let username = user.id().as_str();
    Json(String::from(username))
}
