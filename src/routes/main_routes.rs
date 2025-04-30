use crate::controllers::message::handle_post_message;
use axum::Router;
use axum::routing::post;

pub fn get_routes() -> Router {
    Router::new().route("/debug", post(handle_post_message()))
}
