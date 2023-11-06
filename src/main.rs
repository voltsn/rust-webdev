mod handlers;
use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
};
use crate::handlers::index;

#[tokio::main]
async fn main() {
    // Define router
    let app = Router::new().route("/", get(index));

    // Define server address and listening port
    let server_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("->> Running server on {}", server_addr);
    axum::Server::bind(&server_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
