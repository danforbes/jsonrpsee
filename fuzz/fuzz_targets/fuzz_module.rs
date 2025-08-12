#![no_main]

use jsonrpsee::server::RpcModule;

use jsonrpsee_fuzz::ArbitraryJson;

use libfuzzer_sys::fuzz_target;

use serde_json::Value;

use tokio::runtime::Runtime;

fuzz_target!(|input: ArbitraryJson| {
	let rt = Runtime::new().unwrap();
	let mut module = RpcModule::new(());
	module.register_method("fuzz", |params, _, _| params.parse::<Value>()).unwrap();
	rt.block_on(module.call::<ArbitraryJson, Value>("fuzz", input)).unwrap();
});
