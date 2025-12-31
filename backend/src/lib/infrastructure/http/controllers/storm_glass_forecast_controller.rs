use crate::{
    application::use_cases::storm_glass::waves_forecast::StormGlassForecast,
    domain::{
        entities::{coordinates::Coordinate, forecast::storm_glass_waves::StormGlassWavesData},
        port::{cache_api::CacheApi, forecast_provider::ForecastError},
    },
};

pub struct StormGlassController<Cache: CacheApi + Send + Sync> {
    use_cases: StormGlassForecast<Cache>,
}

impl<Cache: CacheApi + Send + Sync> StormGlassController<Cache> {
    pub(crate) fn new(use_cases: StormGlassForecast<Cache>) -> Self {
        Self { use_cases }
    }

    pub(crate) async fn forecast(
        &self,
        coordinates: Coordinate,
    ) -> Result<Vec<StormGlassWavesData>, ForecastError> {
        self.use_cases.execute(coordinates).await
    }
}
