use axum::{
    Json,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::{MethodRouter, post},
};
use futures_util::sink::SinkExt;
use futures_util::stream::{SplitSink, SplitStream, StreamExt}; // Explicitly import SinkExt

pub fn handle_post_message() -> MethodRouter {
    post(|Json(event): Json<serde_json::Value>| async move {
        println!("Received event: {:?}", event);
        println!("{:?}", event["helloo"].to_string());
        "Event received"
    })
}

pub async fn handle_socket(socket: WebSocket) {
    let (sender, receiver) = socket.split();

    // Pass both sender and receiver to the read function
    tokio::spawn(handle_ws_message(receiver, sender));
}

pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_ws_message(
    mut receiver: SplitStream<WebSocket>,
    mut sender: SplitSink<WebSocket, Message>,
) {
    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(text) => {
                println!("Received text: {}", text);

                // Send a response back to the client
                if let Err(e) = sender
                    .send(Message::Text(format!("Server received: {}", text).into()))
                    .await
                {
                    println!("Failed to send response: {}", e);
                }
            }
            Message::Binary(_) => {
                println!("Received binary data");
            }
            _ => (),
        }
    }
    println!("receiver: {:#?}", receiver);
}
