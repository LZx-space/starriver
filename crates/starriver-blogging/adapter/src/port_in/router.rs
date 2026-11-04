use axum::{
    Router,
    routing::{get, post},
};

use crate::port_in::{attachment_handler, category_handler, post_handler, state::BloggingState};

pub fn create_router(state: BloggingState) -> Router<()> {
    Router::new()
        .route(
            "/categories",
            get(category_handler::list).post(category_handler::create),
        )
        .route(
            "/categories/{id}",
            get(category_handler::find)
                .put(category_handler::update)
                .delete(category_handler::delete),
        )
        .route(
            "/posts",
            get(post_handler::paginate).post(post_handler::create),
        )
        .route(
            "/posts/{id}",
            get(post_handler::find)
                .put(post_handler::update)
                .delete(post_handler::delete),
        )
        .route("/attachments", post(attachment_handler::upload))
        .route("/search/posts", get(post_handler::search))
        .with_state(state)
}
