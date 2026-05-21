use axum::{
    Router,
    routing::{get, post, put},
};

use crate::port_in::{state::IdentityState, user_handler};

pub fn create_router(state: IdentityState) -> impl Into<Router> {
    Router::new()
        .route("/users/me", get(user_handler::me))
        .route("/users", post(user_handler::register_user))
        .route("/users/{username}/state", put(user_handler::activate_user))
        .route(
            "/email-verifications",
            post(user_handler::send_register_email),
        )
        .with_state(state)
}
