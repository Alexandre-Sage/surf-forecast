
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::api_key::ApiKey;

#[derive(Debug, thiserror::Error)]
pub enum ApiKeyRepositoryError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    ChronoDateConversion(#[from] chrono::ParseError),
    #[error(transparent)]
    UuidConversion(#[from] uuid::Error),
}

#[async_trait]
pub(crate) trait ApiKeyRepository: Send + Sync {
    async fn api_keys(&self) -> Result<Vec<ApiKey>, ApiKeyRepositoryError>;
    async fn update_usage_count(&self, id: Uuid, count: u8) -> Result<(), ApiKeyRepositoryError>;
    async fn get_available_key(
        &self,
        max_usage: u8,
    ) -> Result<Option<ApiKey>, ApiKeyRepositoryError>;
    async fn batch_reset_usage(&self, keys_id: Vec<Uuid>) -> Result<(), ApiKeyRepositoryError>;
}
