use tokio::sync::mpsc;


pub async fn receive_messages(mut rx: mpsc::Receiver<String>) {
    while let Some(msg) = rx.recv().await {
        println!("Received Message: {}", msg);
    }
}