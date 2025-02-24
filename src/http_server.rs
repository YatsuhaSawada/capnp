use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use std::thread;
use std::sync::Arc;
use tokio::sync::Mutex;
pub async fn start(tx: Arc<Mutex<mpsc::Sender<String>>>) {
    let tx_clone = Arc::clone(&tx);
    let app = Router::new().route("/", get(move|| { 
        let tx = Arc::clone(&tx_clone);

        async move {
            let r = "Hello, World!";
            let tid = thread::current().id();
            let message = format!("{} TID={:?}\n", r, tid);
            let mut tx = tx.lock().await;
            if let Err(_) = tx.send(message.clone()).await {
                eprintln!("Failed to send message");
            }
            message
        }
     }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
