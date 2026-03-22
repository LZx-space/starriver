use axum::Json;
use axum::response::IntoResponse;
use starriver_infrastructure::security::authentication::_default_impl::AuthenticatedUser;

pub async fn me(user: AuthenticatedUser) -> impl IntoResponse {
    Json(user)
}
