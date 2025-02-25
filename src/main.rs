// Copyright (c) 2013-2015 Sandstorm Development Group, Inc. and contributors
// Licensed under the MIT License:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use std::sync::Arc;
use tokio::sync::Mutex;
pub mod hello_world_capnp {
    include!(concat!(env!("OUT_DIR"), "/hello_world_capnp.rs"));
}
pub mod echo_capnp {
    include!(concat!(env!("OUT_DIR"), "/echo_capnp.rs"));
}


pub mod client;
pub mod http_server;
pub mod server;
pub mod message;

use tokio::sync::mpsc;
use crate::message::receive_messages;

//#[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (tx, mut rx) = mpsc::channel(32);

    let handle = tokio::spawn(receive_messages(rx));

    let tx_a = Arc::new(Mutex::new(tx.clone()));
    let (_, _, _) = tokio::join!(server::main(tx.clone()), http_server::start(tx_a), handle);

    Ok(())
}
/*use tokio::runtime::Runtime;
use tokio::task::LocalSet;
use once_cell::sync::Lazy;
use std::thread;

// `tokio::runtime::Runtime`を`Lazy`で保持
static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Runtime::new().unwrap()
});

async fn async_function() {
    // 非同期関数の実装
    println!("This is an async 11");
}

fn read() {
    // 別スレッドで非同期タスクを実行
    let handle = thread::spawn(|| {
        let local = LocalSet::new();
        
        local.spawn_local(async {
            async_function().await;
        });

        // LocalSetを実行
        RUNTIME.block_on(local)
    });

    // スレッドの終了を待つ
    handle.join().unwrap();
}

fn main() {
    // read関数を呼び出し
    RUNTIME.block_on(async {
        read();
    });
}*/

/*
use tokio::runtime::Runtime;
use tokio::task::LocalSet;
use tokio::time::{sleep, Duration};
use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ClientImpl {
    local: LocalSet,
    rt: Arc<Runtime>,
}

impl ClientImpl {
    pub fn new(rt: Arc<Runtime>, local: LocalSet) -> Self {
        ClientImpl { local, rt }
    }

    async fn async_read(&self) {
        // 別crateのLocalでしか実行できない関数を呼び出す
        println!("Reading data in the LocalSet");
        sleep(Duration::from_secs(1)).await;
        println!("Read completed");
    }

    async fn async_write(&self) {
        // 別crateのLocalでしか実行できない関数を呼び出す
        println!("Writing data in the LocalSet");
        sleep(Duration::from_secs(1)).await;
        println!("Write completed");
    }

    pub fn read(&self) {
        let local = &self.local;
        self.rt.block_on(local.run_until(async {
            tokio::task::spawn_local(async {
                self.async_read().await;
            }).await.unwrap();
        }));
    }

    pub fn write(&self) {
        let local = &self.local;
        self.rt.block_on(local.run_until(async {
            tokio::task::spawn_local(async {
                self.async_write().await;
            }).await.unwrap();
        }));
    }
}

struct Client;

impl client_capnp::client::Server for Client {
    fn read(
        &mut self,
        _params: client_capnp::client::ReadParams,
        mut results: client_capnp::client::ReadResults,
    ) -> Promise<(), capnp::Error> {
        let client = self.client.clone();
        Promise::from_future(async move {
            client.lock().await.read();
            Ok(())
        })
    }

    fn write(
        &mut self,
        _params: client_capnp::client::WriteParams,
        mut results: client_capnp::client::WriteResults,
    ) -> Promise<(), capnp::Error> {
        let client = self.client.clone();
        Promise::from_future(async move {
            client.lock().await.write();
            Ok(())
        })
    }
}

fn main() {
    // Create a new runtime
    let rt = Arc::new(Runtime::new().unwrap());

    // Create a new LocalSet
    let local = LocalSet::new();

    let client_impl = Arc::new(Mutex::new(ClientImpl::new(rt.clone(), local)));

    // Cap'n Protoのサーバに接続してリクエストを処理する
    rt.block_on(async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await.unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

        let network = twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Server,
            Default::default(),
        );

        let client = Client { client: client_impl };

        let mut rpc_system = RpcSystem::new(Box::new(network), Some(client.clone().client));

        tokio::task::spawn_local(rpc_system.map_err(|e| eprintln!("rpc error: {:?}", e)));
    });
}*/

/*
use tokio::runtime::Runtime;
use tokio::task::LocalSet;
use tokio::time::{sleep, Duration};
use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ClientImpl {
    local: LocalSet,
    rt: Arc<Runtime>,
}

impl ClientImpl {
    pub fn new(rt: Arc<Runtime>, local: LocalSet) -> Self {
        ClientImpl { local, rt }
    }

    async fn async_read(&self) {
        // 別crateのLocalでしか実行できない関数を呼び出す
        println!("Reading data in the LocalSet");
        sleep(Duration::from_secs(1)).await;
        println!("Read completed");
    }

    async fn async_write(&self) {
        // 別crateのLocalでしか実行できない関数を呼び出す
        println!("Writing data in the LocalSet");
        sleep(Duration::from_secs(1)).await;
        println!("Write completed");
    }

    pub async fn read(&self) {
        self.local.run_until(async {
            tokio::task::spawn_local(async {
                self.async_read().await;
            }).await.unwrap();
        }).await;
    }

    pub async fn write(&self) {
        self.local.run_until(async {
            tokio::task::spawn_local(async {
                self.async_write().await;
            }).await.unwrap();
        }).await;
    }
}

struct Client {
    client: Arc<Mutex<ClientImpl>>,
}

impl client_capnp::client::Server for Client {
    fn read(
        &mut self,
        _params: client_capnp::client::ReadParams,
        mut results: client_capnp::client::ReadResults,
    ) -> Promise<(), capnp::Error> {
        let client = self.client.clone();
        Promise::from_future(async move {
            client.lock().await.read().await;
            Ok(())
        })
    }

    fn write(
        &mut self,
        _params: client_capnp::client::WriteParams,
        mut results: client_capnp::client::WriteResults,
    ) -> Promise<(), capnp::Error> {
        let client = self.client.clone();
        Promise::from_future(async move {
            client.lock().await.write().await;
            Ok(())
        })
    }
}

fn main() {
    // Create a new runtime
    let rt = Arc::new(Runtime::new().unwrap());

    // Create a new LocalSet
    let local = LocalSet::new();

    let client_impl = Arc::new(Mutex::new(ClientImpl::new(rt.clone(), local)));

    // Cap'n Protoのサーバに接続してリクエストを処理する
    rt.block_on(async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:4000").await.unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

        let network = twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Server,
            Default::default(),
        );

        let client = Client { client: client_impl };

        let rpc_system = RpcSystem::new(Box::new(network), Some(client.clone().client));

        tokio::task::spawn_local(rpc_system.map_err(|e| eprintln!("rpc error: {:?}", e))).await.unwrap();
    });
}*/