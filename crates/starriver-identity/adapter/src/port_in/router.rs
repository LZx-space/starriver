use axum::{
    Router,
    routing::{get, post},
};

use crate::port_in::{state::IdentityState, user_handler};

pub fn create_router(state: IdentityState) -> impl Into<Router> {
    Router::new()
        .route("/users/me", get(user_handler::me))
        .route("/users", post(user_handler::register_user))
        .route("/email-verifications", post(user_handler::verify_email))
        .with_state(state)
}
