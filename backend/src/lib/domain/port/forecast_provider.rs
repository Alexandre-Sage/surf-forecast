use ::serde::Deserialize;
use async_trait::async_trait;

use crate::domain::{
    entities::coordinates::Coordinate, port::cache_api::CacheApiError,
    services::api_keys_service::ApiKeyError,
};

pub type ForecastResult<T> = Result<T, ForecastError>;

#[derive(thiserror::Error, Debug)]
pub enum ForecastError {
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("{0}")]
    PaymentRequired(String),
    #[error("{0}")]
    Unknown(String),
    #[error(transparent)]
    ClientError(#[from] reqwest::Error),
    #[error(transparent)]
    UrlError(#[from] url::ParseError),
    #[error(transparent)]
    ApiKeyError(#[from] ApiKeyError),
    #[error(transparent)]
    Cache(#[from] CacheApiError),
}

// impl From<StatusCode> for ForecastError {}

#[async_trait]
pub trait WavesForecastProvider: Sync + Send {
    type Return;
    async fn waves_forecast(&self, coordinates: &Coordinate) -> ForecastResult<Self::Return>;
}

#[async_trait]
pub(crate) trait WindsForecastProvider {
    async fn winds_forecast<T>(&self) -> ForecastResult<T>
    where
        T: for<'de> Deserialize<'de>;
}
