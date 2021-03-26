use ahash::AHashMap as HashMap;
use serde::{Deserialize, Serialize};

use firecore_input::{Control, KeySetSerializable};

#[derive(Serialize, Deserialize)]
pub struct Configuration {

	pub controls: HashMap<Control, KeySetSerializable>,	
	// pub touchscreen: bool,

}