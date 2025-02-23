use crate::hello_world_capnp::hello_world;
use capnp_rpc::{RpcSystem, rpc_twoparty_capnp, twoparty};
use futures::AsyncReadExt;
use tokio::net::UnixStream;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = "/tmp/hello_world.sock";
    let msg = "Alice".to_string();

    tokio::task::LocalSet::new()
        .run_until(async move {
            let stream = UnixStream::connect(socket_path).await?;
            let (reader, writer) =
                tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
            let rpc_network = Box::new(twoparty::VatNetwork::new(
                futures::io::BufReader::new(reader),
                futures::io::BufWriter::new(writer),
                rpc_twoparty_capnp::Side::Client,
                Default::default(),
            ));
            let mut rpc_system = RpcSystem::new(rpc_network, None);
            let hello_world: hello_world::Client =
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

            tokio::task::spawn_local(rpc_system);

            let mut request = hello_world.say_hello_request();
            request.get().init_request().set_name(&msg[..]);

            let reply = request.send().promise.await?;

            println!(
                "received: {}",
                reply.get()?.get_reply()?.get_message()?.to_str()?
            );
            Ok(())
        })
        .await
}
