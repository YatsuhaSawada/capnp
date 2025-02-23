use axum::{Router, routing::get};
use std::net::SocketAddr;

pub async fn start() {
    let app = Router::new().route("/", get(|| async { "Hello, World!\n" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
