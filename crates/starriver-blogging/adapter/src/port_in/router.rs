use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};

use crate::port_in::{attachment_handler, category_handler, post_handler, state::BloggingState};

pub fn create_router(state: BloggingState) -> Router<()> {
    Router::new()
        .route(
            "/categories",
            get(category_handler::list_all).post(category_handler::create),
        )
        .route(
            "/categories/{id}",
            get(category_handler::show)
                .put(category_handler::update)
                .delete(category_handler::delete),
        )
        .route(
            "/posts",
            get(post_handler::paginate).post(post_handler::create),
        )
        .route(
            "/posts/{id}",
            get(post_handler::show)
                .put(post_handler::update)
                .delete(post_handler::delete),
        )
        .route("/attachments", post(attachment_handler::upload_attachment))
        .route("/search/posts", get(post_handler::search))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .with_state(state)
}
