use async_trait::async_trait;

#[derive(Debug)]
pub enum ForecastError {
    Uncontrolled(String),
    StormGlassApiKeyMaxRequestReached(String),
    CacheError(String),
}
#[async_trait]
pub trait ForecastRepository {
    type WeatherForecast;
    type TideForecast;
    async fn weather_forecast(
        &self,
        lat: f64,
        lng: f64,
    ) -> Result<Self::WeatherForecast, ForecastError>;
    async fn tide_forecast(&self, lat: f64, lng: f64) -> Result<Self::TideForecast, ForecastError>;
}
