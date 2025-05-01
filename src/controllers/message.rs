use axum::{
    Json,
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
    routing::{MethodRouter, post},
};

pub fn handle_post_message() -> MethodRouter {
    post(|Json(event): Json<serde_json::Value>| async move {
        println!("Received event: {:?}", event);
        println!("{:?}", event["helloo"].to_string());
        "Event received"
    })
}

pub async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            println!("WS : Received message: {:?}", msg);
            msg
        } else {
            // client disconnected
            println!("WS : Client disconnected");
            return;
        };

        if socket.send(msg.clone()).await.is_ok() {
            println!("WS : Sent message: {:?}", msg);
        } else {
            // client disconnected
            println!("WS : Client disconnected");
            return;
        }
    }
}

pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}
