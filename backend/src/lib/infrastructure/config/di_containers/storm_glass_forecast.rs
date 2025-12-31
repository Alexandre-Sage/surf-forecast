use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::sync::Mutex;

use crate::{
    application::use_cases::storm_glass::waves_forecast::StormGlassForecast,
    domain::{
        entities::forecast::storm_glass_waves::StormGlassWavesData,
        port::forecast_provider::WavesForecastProvider,
        services::api_keys_service::{ApiKeyError, ApiKeysService},
    },
    infrastructure::{
        cache::redis_cache_api::RedisCacheApi,
        external_service::storm_glass::storm_glass_provider::StormGlassProvider,
        http::controllers::storm_glass_forecast_controller::StormGlassController,
        persistence::sqlite::api_keys::sqlite_api_keys_repository::SqliteApiKeysRepository,
    },
};
pub struct StormGlassForecastDiContainer {
    cache_api: Arc<RedisCacheApi>,
    storm_glass_provider: Arc<dyn WavesForecastProvider<Return = Vec<StormGlassWavesData>>>,
}

impl StormGlassForecastDiContainer {
    pub async fn new(
        db_pool: Arc<SqlitePool>,
        cache_api: Arc<RedisCacheApi>,
    ) -> Result<Self, ApiKeyError> {
        let api_key_repository = Arc::new(SqliteApiKeysRepository::new(Arc::clone(&db_pool)));

        let api_keys_service = Arc::new(Mutex::new(
            ApiKeysService::try_new(api_key_repository).await?,
        ));

        let storm_glass_provider: Arc<
            dyn WavesForecastProvider<Return = Vec<StormGlassWavesData>>,
        > = Arc::new(StormGlassProvider::new(Arc::clone(&api_keys_service)));

        Ok(Self {
            cache_api,
            storm_glass_provider,
        })
    }

    pub fn cache_api(&self) -> Arc<RedisCacheApi> {
        self.cache_api.clone()
    }

    pub fn use_case(&self) -> StormGlassForecast<RedisCacheApi> {
        StormGlassForecast::new(Arc::clone(&self.storm_glass_provider), self.cache_api())
    }

    pub fn controller(&self) -> StormGlassController<RedisCacheApi> {
        StormGlassController::new(self.use_case())
    }
}
