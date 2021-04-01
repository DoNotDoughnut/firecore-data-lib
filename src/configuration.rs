use ahash::AHashMap as HashMap;
use serde::{Deserialize, Serialize};

use firecore_input::{Control, KeySetSerializable};

#[derive(Serialize, Deserialize)]
pub struct Configuration {

	#[serde(default)]
	pub controls: HashMap<Control, KeySetSerializable>,	

	#[serde(default)]
	pub touchscreen: bool,

}