use std::io::Read;

use arbitrary::Arbitrary;

use jsonrpsee_fuzz::ArbitraryJson;

use serde_json::Value;

use tokio::runtime::Runtime;

fn main() -> anyhow::Result<()> {
	let args: Vec<String> = std::env::args().collect();
	let filename = &args[1];

	let mut f = std::fs::File::open(filename)?;
	let mut buffer = Vec::new();
	f.read_to_end(&mut buffer)?;

	let data = ArbitraryJson::arbitrary(&mut arbitrary::Unstructured::new(&buffer))?;

	let mut module = jsonrpsee::server::RpcModule::new(());
	module.register_method("fuzz", |params, _, _| params.parse::<Value>())?;

	let rt = Runtime::new().unwrap();
	rt.block_on(module.call::<ArbitraryJson, Value>("fuzz", data))?;

	Ok(())
}
