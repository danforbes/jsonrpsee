#![no_main]

use jsonrpsee::server::RpcModule;

use jsonrpsee_fuzz::ArbitraryJson;

use libfuzzer_sys::fuzz_target;

use tokio::runtime::Runtime;

fuzz_target!(|input: ArbitraryJson| {
	let rt = Runtime::new().unwrap();
	let mut module = RpcModule::new(());
	module.register_method("boo", |_, _, _| String::from("boo!")).unwrap();
	rt.block_on(module.call::<ArbitraryJson, String>("boo", input)).unwrap();
});
