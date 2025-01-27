use async_trait::async_trait;
use futures::TryFutureExt;
use uuid::Uuid;

use crate::domain::{
    port::spot_repository::{SpotError, SpotRepository},
    r#type::spot::Spot,
};

use super::postgres_repository::PostgresRepository;

#[async_trait]
impl SpotRepository for PostgresRepository {
    async fn all(&self) -> Result<Vec<Spot>, SpotError> {
        const QUERY: &str = "SELECT * FROM spots;";
        sqlx::query_as(QUERY)
            .fetch_all(&self.pool)
            .map_err(|e| SpotError::Uncontrolled(e.to_string()))
            .await
    }
    async fn by_id(&self, id: Uuid) -> Result<Option<Spot>, SpotError> {
        const QUERY: &str = "SELECT * FROM spots WHERE id = $1;";
        sqlx::query_as(QUERY)
            .bind(id)
            .fetch_optional(&self.pool)
            .map_err(|e| SpotError::Uncontrolled(e.to_string()))
            .await
    }
}
