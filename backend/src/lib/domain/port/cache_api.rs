use std::future::Future;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
#[error("Cache error")]
pub enum CacheApiError {
    #[error(transparent)]
    Redis(#[from] redis::RedisError),
    #[error(transparent)]
    SerDe(#[from] serde_json::Error),
    #[error("{0}")]
    Callback(String),
}

#[async_trait]
pub trait CacheApi: Send + Sync {
    async fn get_or_set<Return, Promise, Fut, Error>(
        &self,
        key: &str,
        cb: Promise,
    ) -> Result<Return, CacheApiError>
    where
        Return: Serialize + for<'de> Deserialize<'de> + Send + Sync,
        Promise: Fn() -> Fut + Send + Sync,
        Fut: Future<Output = Result<Return, Error>> + Send,
        Error: ToString + std::fmt::Display + Send + Sync;
}
