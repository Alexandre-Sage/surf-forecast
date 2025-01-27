use async_trait::async_trait;

pub enum ForecastError {
    Uncontrolled(String),
}
#[async_trait]
pub trait ForecastRepository {
    type WeatherForecast;
    type TideForecast;
    async fn weather_forecast() -> Result<Self::WeatherForecast, ForecastError>;
    async fn tide_forecast() -> Result<Self::TideForecast, ForecastError>;
}
