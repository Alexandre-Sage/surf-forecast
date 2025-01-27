use crate::domain::r#type::spot::Spot;
use async_trait::async_trait;
use internal::error::api::ApiError;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub enum SpotError {
    Uncontrolled(String),
    NotFound,
}

impl From<SpotError> for ApiError {
    fn from(value: SpotError) -> Self {
        match value {
            SpotError::NotFound => Self::UnprocessableEntity("SPOT_NOT_FOUND".to_string()),
            SpotError::Uncontrolled(err) => Self::InternalServerError(err),
        }
    }
}

#[async_trait]
pub trait SpotRepository {
    async fn all(&self) -> Result<Vec<Spot>, SpotError>;
    async fn by_id(&self, id: Uuid) -> Result<Option<Spot>, SpotError>;
}
