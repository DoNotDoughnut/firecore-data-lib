use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod player;

pub trait PersistantData: Serialize + DeserializeOwned + Default {

    fn file_name() -> &'static str;

}