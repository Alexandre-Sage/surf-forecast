use std::sync::Arc;

use serde::Deserialize;
use sqlx::SqlitePool;

use crate::infrastructure::{
    cache::redis_cache_api::RedisCacheApi,
    config::di_containers::{
        storm_glass_forecast::StormGlassForecastDiContainer,
        wave_calculation::WaveCalculationContainer,
    },
    http::{
        controllers::{
            storm_glass_forecast_controller::StormGlassController,
            wave_calculation_controller::WaveCalculationController,
        },
        server::BootError,
    },
};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
    pub sqlite_url: String,
    pub redis_url: String,
}

pub struct AppState {
    pub(crate) storm_glass_forecast: StormGlassController<RedisCacheApi>,
    pub(crate) wave_calculation: WaveCalculationController,
}

impl AppState {
    pub async fn try_from(value: &ServerConfig) -> Result<Self, BootError> {
        let redis = redis::Client::open(value.redis_url.as_str())?;

        let cache = RedisCacheApi::new(Arc::new(redis)).into();
        let pool = SqlitePool::connect(
            &value.sqlite_url, //"sqlie:sqlite-databases/api-keys.sqlite"
        )
        .await?
        .into();

        let storm_glass_forecast = StormGlassForecastDiContainer::new(pool, cache).await?;
        let wave_calculation = WaveCalculationContainer::new();

        Ok(Self {
            storm_glass_forecast: storm_glass_forecast.controller(),
            wave_calculation: wave_calculation.controller(),
        })
    }
}
