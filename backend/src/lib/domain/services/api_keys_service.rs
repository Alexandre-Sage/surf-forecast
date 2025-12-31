use std::sync::Arc;

use chrono::Utc;
use log::info;

use crate::domain::{
    entities::api_key::ApiKey,
    port::api_key_repository::{ApiKeyRepository, ApiKeyRepositoryError},
};

#[derive(Debug, thiserror::Error)]
pub enum ApiKeyError {
    #[error(transparent)]
    Repository(#[from] ApiKeyRepositoryError),
    #[error("All keys have reach max daily usage")]
    AllKeyReachedMaxLimit,
}

pub struct ApiKeysService {
    api_keys: Vec<ApiKey>,
    api_key_repository: Arc<dyn ApiKeyRepository>,
    current: ApiKey,
}

impl ApiKeysService {
    pub(crate) async fn try_new(
        api_key_repository: Arc<dyn ApiKeyRepository>,
    ) -> Result<Self, ApiKeyError> {
        let mut api_keys = api_key_repository.api_keys().await?;

        let keys_to_reset: Vec<_> = api_keys
            .iter()
            .filter(|item| item.date().date_naive() != Utc::now().date_naive())
            .map(|item| item.id)
            .collect();

        if !keys_to_reset.is_empty() {
            api_key_repository.batch_reset_usage(keys_to_reset).await?;
            api_keys.iter_mut().for_each(|key| key.reset());
        }

        let current = api_keys
            .iter()
            .find(|key| key.daily_usage() < 10)
            .ok_or(ApiKeyError::AllKeyReachedMaxLimit)?
            .clone();

        Ok(Self {
            api_keys,
            api_key_repository,
            current,
        })
    }
    pub fn current(&self) -> &str {
        self.current.key()
    }

    pub async fn increment_usage(&mut self) -> Result<(), ApiKeyError> {
        self.current.increment_usage();

        let maybe_key = self
            .api_keys
            .iter_mut()
            .find(|key| key.id == self.current.id);

        if let Some(key) = maybe_key {
            *key = self.current.clone()
        }
        self.api_key_repository
            .update_usage_count(self.current.id, self.current.daily_usage())
            .await?;

        Ok(())
    }

    pub fn need_rotation(&self) -> bool {
        self.current.daily_usage() >= 10
    }

    pub fn rotate(&mut self) -> Result<(), ApiKeyError> {
        info!("Rotate storm glass key needed");
        let key = self
            .api_keys
            .iter()
            .find(|key| key.daily_usage() < 10)
            .ok_or(ApiKeyError::AllKeyReachedMaxLimit)?;

        self.current = key.clone();
        info!("Storm glass key rotation successfull");
        Ok(())
    }
}
