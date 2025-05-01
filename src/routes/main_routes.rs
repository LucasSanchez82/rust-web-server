use crate::controllers::message::{handle_post_message, handler};
use axum::Router;
use axum::routing::{any, post};

pub fn get_routes() -> Router {
    Router::new()
        .route("/debug", post(handle_post_message()))
        .route("/ws", any(handler))
}
