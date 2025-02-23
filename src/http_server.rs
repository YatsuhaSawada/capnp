use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use std::thread;
pub async fn start(tx: mpsc::Sender<String>) {
    let app = Router::new().route("/", get(|| async { 
        let r = "Hello, World!\n";
        let tid = thread::current().id();
        let message = format!("{}! TID={:?}", r, tid);
        message
     }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
