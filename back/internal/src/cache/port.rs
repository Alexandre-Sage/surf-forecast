use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Cache {
    type Error: std::fmt::Debug;
    async fn set<T>(&mut self, key: &str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + Send + Sync;

    async fn get<T>(&self, key: &str) -> Result<Option<T>, Self::Error>
    where
        T: for<'de> Deserialize<'de> + Send + Sync;

    async fn get_or_default<T>(&mut self, key: &str, default: T) -> Result<T, Self::Error>
    where
        T: for<'de> Deserialize<'de> + Serialize + Send + Sync;

    async fn delete(&mut self, key: &str) -> Result<(), Self::Error>;
}
