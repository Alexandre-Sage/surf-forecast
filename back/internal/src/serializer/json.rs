use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::port::{DeserializationError, SerializationError, Serializer};
pub struct JsonSerializer;

impl Serializer for JsonSerializer {
    fn serialize<T>(&self, value: &T) -> Result<Vec<u8>, SerializationError>
    where
        T: Serialize,
    {
        serde_json::to_vec(value).map_err(|_| SerializationError::Uncontrolled)
    }
    fn deserialize<T>(&self, value: impl AsRef<[u8]>) -> Result<T, DeserializationError>
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_json::from_slice(value.as_ref()).map_err(|_| DeserializationError::Uncontrolled)
    }
    fn deserialize_to_owned<T>(&self, value: impl AsRef<[u8]>) -> Result<T, DeserializationError>
    where
        T: DeserializeOwned,
    {
        serde_json::from_slice(value.as_ref()).map_err(|_| DeserializationError::Uncontrolled)
    }
}
