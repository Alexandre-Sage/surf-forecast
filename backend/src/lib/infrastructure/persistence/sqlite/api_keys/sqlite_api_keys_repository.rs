use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use log::info;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    domain::{
        entities::api_key::ApiKey,
        port::api_key_repository::{ApiKeyRepository, ApiKeyRepositoryError},
    },
    infrastructure::persistence::sqlite::api_keys::models::ApiKeySelectModel,
};

pub(crate) struct SqliteApiKeysRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteApiKeysRepository {
    pub(crate) fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ApiKeyRepository for SqliteApiKeysRepository {
    async fn api_keys(&self) -> Result<Vec<ApiKey>, ApiKeyRepositoryError> {
        info!("Selecting all api keys");
        let rows = sqlx::query_as!(ApiKeySelectModel, "select * from api_keys")
            .fetch_all(&*self.pool)
            .await?;

        let api_keys = rows.into_iter().map(ApiKey::try_from).collect();
        info!("Selecting all api keys success");

        api_keys
    }

    async fn update_usage_count(&self, id: Uuid, count: u8) -> Result<(), ApiKeyRepositoryError> {
        info!("Updating api key with id {} to count: {}", id, count);
        let count = count as i64;
        let id = id.to_string();

        sqlx::query!(
            "update api_keys set daily_usage = ? where id = ?",
            count,
            id
        )
        .execute(&*self.pool)
        .await?;
        info!("Updating api key with id {} success", id);

        Ok(())
    }

    async fn batch_reset_usage(&self, keys_id: Vec<Uuid>) -> Result<(), ApiKeyRepositoryError> {
        if keys_id.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = (0..keys_id.len()).map(|_| "?".to_string()).collect();
        let placeholders = placeholders.join(", ");
        let query_str = format!(
            "update api_keys set date = ?, daily_usage = 0 where id in ({})",
            placeholders
        );

        let keys_id: Vec<String> = keys_id
            .into_iter()
            .map(|key_id| key_id.to_string())
            .collect();

        let mut base_query = sqlx::query(&query_str).bind(Utc::now());

        for key_id in keys_id {
            base_query = base_query.bind(key_id)
        }

        base_query.execute(&*self.pool).await?;

        Ok(())
    }

    async fn get_available_key(
        &self,
        max_usage: u8,
    ) -> Result<Option<ApiKey>, ApiKeyRepositoryError> {
        let row = sqlx::query_as!(
            ApiKeySelectModel,
            "select * from api_keys where daily_usage < ?",
            max_usage
        )
        .fetch_optional(&*self.pool)
        .await?;

        if let Some(key) = row {
            let api_key = ApiKey::try_from(key)?;
            return Ok(Some(api_key));
        }

        Ok(None)
    }
}
