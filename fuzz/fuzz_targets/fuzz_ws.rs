#![no_main]

use std::net::SocketAddr;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::server::{RpcModule, Server};
use jsonrpsee::ws_client::WsClientBuilder;

use jsonrpsee_fuzz::ArbitraryJson;

use libfuzzer_sys::fuzz_target;

use tokio::runtime::Runtime;

fuzz_target!(|input: ArbitraryJson| {
	let rt = Runtime::new().unwrap();
	let server_addr = rt.block_on(run_server()).unwrap();
	let url = format!("ws://{}", server_addr);
	let client = rt.block_on(WsClientBuilder::new().build(&url)).unwrap();
	rt.block_on(client.request::<String, _>("say_hello", input)).unwrap();
});

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = Server::builder().build("127.0.0.1:0".parse::<SocketAddr>()?).await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _, _| "lo")?;

	let addr = server.local_addr()?;
	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
