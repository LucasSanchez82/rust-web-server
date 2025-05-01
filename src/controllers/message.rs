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

    tokio::spawn(write(sender));
    tokio::spawn(read(receiver));
}

pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn read(mut receiver: SplitStream<WebSocket>) {
    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(text) => println!("Received text: {}", text),
            Message::Binary(_) => println!("Received binary data"),
            _ => (),
        }
    }
    println!("receiver: {:#?}", receiver);
}

async fn write(mut sender: SplitSink<WebSocket, Message>) {
    if let Err(e) = sender.send(Message::Text("Hello from server".into())).await {
        println!("Failed to send message: {}", e);
    }
    println!("sender: {:#?}", sender);
}
