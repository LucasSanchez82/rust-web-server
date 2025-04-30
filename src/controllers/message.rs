use axum::{
    Json,
    routing::{MethodRouter, post},
};

pub fn handle_post_message() -> MethodRouter {
    post(|Json(event): Json<serde_json::Value>| async move {
        println!("Received event: {:?}", event);
        println!("{:?}", event["helloo"].to_string());
        "Event received"
    })
}
