use axum::{
    Json, Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hello", get(|| async { "Hello, Axum!" }))
        .route(
            "/test",
            post(|Json(event): Json<serde_json::Value>| async move {
                println!("Received event: {:?}", event);
                println!("{:?}", event["helloo"].to_string());
                "Event received"
            }),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
