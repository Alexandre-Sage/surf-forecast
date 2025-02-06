use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[derive(Debug)]
pub enum SerializationError {
    Uncontrolled,
}
#[derive(Debug)]
pub enum DeserializationError {
    Uncontrolled,
}

pub trait Serializer {
    fn serialize<T>(&self, value: &T) -> Result<Vec<u8>, SerializationError>
    where
        T: Serialize;

    fn deserialize_to_owned<T>(&self, value: impl AsRef<[u8]>) -> Result<T, DeserializationError>
    where
        T: DeserializeOwned;

    fn deserialize<T>(&self, value: impl AsRef<[u8]>) -> Result<T, DeserializationError>
    where
        T: for<'de> Deserialize<'de>;
}
