use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};

use crate::serializer::{json::JsonSerializer, port::Serializer};

use super::port::Cache;

pub struct RedisCache<S: Serializer> {
    pub client: redis::Client,
    pub serializer: S,
}

impl<S: Serializer> RedisCache<S> {
    pub fn new(client: redis::Client, serializer: S) -> Self {
        Self { client, serializer }
    }

    async fn acquire(&self) -> Result<MultiplexedConnection, RedisError> {
        self.client.get_multiplexed_async_connection().await
    }
}

impl RedisCache<JsonSerializer> {
    pub fn json_cache(client: redis::Client) -> Self {
        Self::new(client, JsonSerializer)
    }
}
#[async_trait]
impl<S: Serializer + Send + Sync> Cache for RedisCache<S> {
    type Error = RedisError;

    async fn set<T>(&mut self, key: &str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize + Send + Sync,
    {
        self.acquire()
            .await?
            .set(key, self.serializer.serialize(value).unwrap())
            .await
    }

    async fn get<T>(&self, key: &str) -> Result<Option<T>, Self::Error>
    where
        T: for<'de> Deserialize<'de> + Send + Sync,
    {
        let value: Option<String> = self.acquire().await?.get(key).await?;
        value
            .map(|value| {
                dbg!(&value);
                serde_json::from_str(&value).map_err(|err| {
                    RedisError::from((
                        redis::ErrorKind::TypeError,
                        "Deserialization error",
                        format!("{}: {}", err, value),
                    ))
                })
            })
            .transpose()
    }

    async fn get_or_default<T>(&mut self, key: &str, default: T) -> Result<T, Self::Error>
    where
        T: for<'de> Deserialize<'de> + Serialize + Send + Sync,
    {
        match self.get(key).await? {
            Some(value) => Ok(value),
            None => {
                self.set(key, &default).await?;
                Ok(default)
            }
        }
    }

    async fn delete(&mut self, key: &str) -> Result<(), Self::Error> {
        self.acquire().await?.del(key).await
    }
}
