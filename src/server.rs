use crate::hello_world_capnp::hello_world;
use capnp::capability::Promise;
use capnp_rpc::{RpcSystem, pry, rpc_twoparty_capnp, twoparty};

use futures::AsyncReadExt;
use std::path::Path;
use tokio::fs;
use tokio::net::UnixListener;
use tokio::sync::mpsc;
use std::thread;
struct HelloWorldImpl{tx: mpsc::Sender<String>}

impl hello_world::Server for HelloWorldImpl {
    fn say_hello(
        &mut self,
        params: hello_world::SayHelloParams,
        mut results: hello_world::SayHelloResults,
    ) -> Promise<(), ::capnp::Error> {
        let request = pry!(pry!(params.get()).get_request());
        let name = pry!(pry!(request.get_name()).to_str());
        let tid = thread::current().id();
        let message = format!("Hello, {}! TID={:?}", name, tid);
        results.get().init_reply().set_message(message.clone());

        //self.tx.send(message.to_string()).await.unwrap();
        let tx = self.tx.clone();
        Promise::<(), ::capnp::Error>::from_future(async move {
            let _ = tx.send(message).await;
            Ok(())
        })
    }
}

pub async fn main(tx: mpsc::Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = "/tmp/hello_world.sock";

    // 既にソケットが存在していたら削除
    if Path::new(socket_path).exists() {
        fs::remove_file(socket_path).await?;
    }

    let listener = UnixListener::bind(socket_path)?;
    let hello_world_client: hello_world::Client = capnp_rpc::new_client(HelloWorldImpl{tx: tx.clone()});

    tokio::task::LocalSet::new()
        .run_until(async move {
            loop {
                println!("Listening on {}", socket_path);
                let (stream, _) = listener.accept().await?;

                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

                let network = twoparty::VatNetwork::new(
                    futures::io::BufReader::new(reader),
                    futures::io::BufWriter::new(writer),
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system =
                    RpcSystem::new(Box::new(network), Some(hello_world_client.clone().client));

                tokio::task::spawn_local(rpc_system);
            }
        })
        .await
}
