use std::{future::Future, sync::Arc};

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::domain::port::cache_api::{CacheApi, CacheApiError};

#[derive(Clone)]
pub struct RedisCacheApi {
    redis_client: Arc<redis::Client>,
}

impl RedisCacheApi {
    pub fn new(redis_client: Arc<redis::Client>) -> Self {
        Self { redis_client }
    }

    pub async fn connection(&self) -> Result<redis::aio::MultiplexedConnection, CacheApiError> {
        self.redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(CacheApiError::from)
    }

    fn seconds_until_midnight() -> u64 {
        let now = Utc::now();
        let tomorrow = (now.date_naive() + Duration::days(1))
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let tomorrow_utc = DateTime::<Utc>::from_naive_utc_and_offset(tomorrow, Utc);
        let duration = tomorrow_utc - now;
        duration.num_seconds().max(1) as u64 // Ensure at least 1 second
    }
}

#[async_trait]
impl CacheApi for RedisCacheApi {
    async fn get_or_set<Return, Promise, Fut, Error>(
        &self,
        key: &str,
        cb: Promise,
    ) -> Result<Return, CacheApiError>
    where
        Return: Serialize + for<'de> Deserialize<'de> + Send + Sync,
        Fut: Future<Output = Result<Return, Error>> + Send,
        Promise: Fn() -> Fut + Send + Sync,
        Error: ToString + std::fmt::Display + Send + Sync,
    {
        let mut connection = self.connection().await?;
        let maybe_cache: Option<String> = connection.get(key).await?;

        return match maybe_cache {
            Some(data) => serde_json::from_str::<Return>(&data).map_err(CacheApiError::from),
            None => {
                let data = cb()
                    .await
                    .map_err(|err| CacheApiError::Callback(err.to_string()))?;

                let serialized_data = json!(&data);
                connection
                    .set_ex::<&str, &str, ()>(
                        key,
                        &serialized_data.to_string(),
                        Self::seconds_until_midnight(),
                    )
                    .await?;

                Ok(data)
            }
        };
    }
}
