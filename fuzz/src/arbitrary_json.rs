use std::ops::Deref;

use arbitrary::Arbitrary;

use arbitrary_json::ArbitraryValue;

use jsonrpsee::core::{JsonRawValue, traits::ToRpcParams};

use serde_json::value::to_raw_value;

#[derive(Arbitrary, Debug)]
pub struct ArbitraryJson(ArbitraryValue);

impl ToRpcParams for ArbitraryJson {
	fn to_rpc_params(self) -> Result<Option<Box<JsonRawValue>>, serde_json::Error> {
		Ok(Some(to_raw_value(self.0.deref())?))
	}
}
