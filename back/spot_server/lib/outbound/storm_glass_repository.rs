use std::{collections::HashMap, i16};

use async_trait::async_trait;
use axum::routing::trace_service;
use futures::{FutureExt, TryFutureExt};
use reqwest::{header, Method, StatusCode};
use tracing::span;

use crate::domain::{
    port::forecast_repository::{ForecastError, ForecastRepository},
    r#type::storm_glass::{
        tide_forecast::StormGlassTideForecast, weather_forecast::StormGlassWeatherForecast,
    },
};
#[derive(Clone)]
pub struct StormGlassKey {
    key: String,
    count: i16,
}
impl StormGlassKey {
    fn new(key: String, count: i16) -> Self {
        Self { key, count }
    }
    pub fn is_valid(&self) -> bool {
        self.count < 10
    }
    pub fn increment(&mut self) {
        self.count += 1
    }
}
impl AsRef<str> for StormGlassKey {
    fn as_ref(&self) -> &str {
        self.key.as_ref()
    }
}
pub struct StormGlassKeys(Vec<StormGlassKey>);

impl StormGlassKeys {
    pub fn new() -> Self {
        Self(
            API_KEYS
                .into_iter()
                .map(|key| StormGlassKey::new(key.to_string(), 0))
                .collect(),
        )
    }
    pub fn select_key(&mut self) -> String {
        self.0
            .iter_mut()
            .find(|key| key.is_valid())
            .map(|key| {
                key.increment();
                key.clone().key
            })
            .unwrap()
    }
}
const API_KEYS: [&str; 5] = [
    "bf91a76a-8545-11ee-8fd1-0242ac130002-bf91a7d8-8545-11ee-8fd1-0242ac130002",
    "9a90a39c-86cd-11ee-8fd1-0242ac130002-9a90a414-86cd-11ee-8fd1-0242ac130002",
    "82cf2668-86df-11ee-8fd1-0242ac130002-82cf26d6-86df-11ee-8fd1-0242ac130002",
    "b772d770-86df-11ee-935b-0242ac130002-b772d81a-86df-11ee-935b-0242ac130002",
    "aa2398a4-b1f3-11ee-950b-0242ac130002-aa239962-b1f3-11ee-950b-0242ac130002",
];

pub struct StormGlassRepository {
    base_url: String,
    api_keys: StormGlassKeys,
}

impl StormGlassRepository {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.stormglass.io".to_string(),
            api_keys: StormGlassKeys::new(),
        }
    }

    fn weather_forecast_url(&self, lat: f64, lng: f64) -> String {
        let query_params =  "airTemperature,cloudCover,currentDirection,precipitation,pressure,seaLevel,secondarySwellDirection,secondarySwellHeight,secondarySwellPeriod,swellDirection,swellHeight,swellPeriod,visibility,waterTemperature,waveDirection,waveHeight,wavePeriod,windDirection,windSpeed,windWaveDirection,windWaveHeight,windWavePeriod";
        format!(
            "{}/v2/weather/point?lat={}&lng={}&params={}",
            self.base_url, lat, lng, query_params
        )
    }

    fn tide_forecast_url(&self, lat: f64, lng: f64, end: &str) -> String {
        format!(
            "{}/v2/tide/extremes/point?lat={}&lng={}&end={}",
            self.base_url, lat, lng, end
        )
    }
}

#[async_trait]
impl ForecastRepository for StormGlassRepository {
    type WeatherForecast = StormGlassWeatherForecast;
    type TideForecast = StormGlassTideForecast;
    async fn weather_forecast(
        &self,
        lat: f64,
        lng: f64,
    ) -> Result<Self::WeatherForecast, ForecastError> {
        let url = self.weather_forecast_url(lat, lng);
        let res = reqwest::Client::new()
            .get(url)
            .header(header::AUTHORIZATION, self.api_keys.select_key())
            .send()
            .map_err(|err| ForecastError::Uncontrolled(err.to_string()))
            .await?;
        match res.status() {
            StatusCode::PAYMENT_REQUIRED => Err(ForecastError::StormGlassApiKeyMaxRequestReached(
                "".to_string(),
            )),
            StatusCode::OK => {
                res.json()
                    .map_err(|err| ForecastError::Uncontrolled(err.to_string()))
                    .await
            }
            _ => Err(ForecastError::Uncontrolled("".to_string())),
        }
    }
    async fn tide_forecast(&self, lat: f64, lng: f64) -> Result<Self::TideForecast, ForecastError> {
        let url = self.tide_forecast_url(lat, lng, "");
        todo!()
    }
}
