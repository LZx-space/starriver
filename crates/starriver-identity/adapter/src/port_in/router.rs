use axum::{Router, routing::get};

use crate::port_in::{state::IdentityState, user_handler};

pub fn create_router(state: IdentityState) -> Router<()> {
    Router::new()
        .route("/users/me", get(user_handler::me))
        .with_state(state)
}
