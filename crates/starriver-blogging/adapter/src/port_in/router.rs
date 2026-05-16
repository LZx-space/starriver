use axum::Router;

use crate::port_in::state::BloggingState;

pub fn create_router(state: BloggingState) -> Router<()> {
    Router::new().with_state(state)
}
