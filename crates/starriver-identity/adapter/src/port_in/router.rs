use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::port_in::{security_event_handler, state::IdentityState, user_handler};

pub fn create_router(state: IdentityState) -> impl Into<Router> {
    Router::new()
        .route("/users/me", get(user_handler::me))
        .route(
            "/users/me/password",
            patch(user_handler::change_my_password),
        )
        .route(
            "/users/email-verification-codes",
            post(user_handler::send_email_verification_code),
        )
        .route(
            "/users",
            get(user_handler::paginate).post(user_handler::register_user),
        )
        .route(
            "/password-reset/verification-codes",
            post(user_handler::send_reset_password_verification_code),
        )
        .route(
            "/password-reset",
            patch(user_handler::reset_password_with_verification_code),
        )
        .route("/security-events", get(security_event_handler::paginate))
        .with_state(state)
}
