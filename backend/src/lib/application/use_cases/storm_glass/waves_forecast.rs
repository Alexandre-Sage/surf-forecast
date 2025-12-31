use std::sync::Arc;


use crate::domain::{
        entities::{coordinates::Coordinate, forecast::storm_glass_waves::StormGlassWavesData},
        port::{
            cache_api::CacheApi,
            forecast_provider::{ForecastError, WavesForecastProvider},
        },
    };

pub(crate) struct StormGlassForecast<Cache: CacheApi + Send + Sync> {
    storm_glass_provider: Arc<dyn WavesForecastProvider<Return = Vec<StormGlassWavesData>>>,
    cache: Arc<Cache>,
}

impl<Cache: CacheApi + Send + Sync> StormGlassForecast<Cache> {
    pub fn new(
        storm_glass_provider: Arc<dyn WavesForecastProvider<Return = Vec<StormGlassWavesData>>>,
        cache: Arc<Cache>,
    ) -> Self {
        Self {
            storm_glass_provider,
            cache,
        }
    }

    pub async fn execute(
        &self,
        coordinates: Coordinate,
    ) -> Result<Vec<StormGlassWavesData>, ForecastError> {
        let cache_key = format!(
            "{}::{}::sg_waves_forecast",
            coordinates.latitude(),
            coordinates.longitude()
        );

        let report = self
            .cache
            .get_or_set(&cache_key, || async {
                let res = self
                    .storm_glass_provider
                    .waves_forecast(&coordinates)
                    .await?;

                Ok::<Vec<StormGlassWavesData>, ForecastError>(res)
            })
            .await?;

        Ok(report)
    }
}
