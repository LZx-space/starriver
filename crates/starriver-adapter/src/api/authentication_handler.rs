use axum::Json;
use axum::response::IntoResponse;
use starriver_infrastructure::security::authentication::core::principal::Principal;
use starriver_infrastructure::security::authentication::username_password_authentication::AuthenticatedUser;

pub async fn authenticated_user(user: AuthenticatedUser) -> impl IntoResponse {
    let username = user.id().as_str();
    Json(String::from(username))
}
