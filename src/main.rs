mod routes {
    pub mod main_routes;
}
pub mod controllers {
    pub mod message;
}

#[tokio::main]
async fn main() {
    const PORT: u16 = 3005;
    let address: String = format!("0.0.0.0:{}", PORT);
    let app = routes::main_routes::get_routes();

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Listening on {}", &address);

    axum::serve(listener, app).await.unwrap();
}
