use std::{
    collections::HashMap,
    i16,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::sync::RwLock;
use tracing_subscriber::filter::FilterExt;

use crate::domain::{
    port::forecast_repository::{ForecastError, ForecastRepository},
    r#type::storm_glass::{
        tide_forecast::StormGlassTideForecast, weather_forecast::StormGlassWeatherForecast,
    },
};
use async_trait::async_trait;
use axum::routing::trace_service;
use futures::{FutureExt, TryFutureExt};
use internal::cache::port::Cache;
use reqwest::{header, Method, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::span;
#[derive(Clone, Serialize, Deserialize)]
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

pub struct StormGlassRepository<C>
where
    C: Cache,
{
    base_url: String,
    cache: Arc<RwLock<C>>,
}

impl<C> StormGlassRepository<C>
where
    C: Cache,
{
    pub fn new(cache: Arc<RwLock<C>>) -> Self {
        Self {
            base_url: "https://api.stormglass.io".to_string(),
            cache,
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

    async fn update_keys(
        &self,
        key: &str,
        keys: &mut HashMap<String, StormGlassKey>,
    ) -> Result<(), ForecastError> {
        if let Some(key) = keys.get_mut(key) {
            key.increment();
        }
        let mut cache = self.cache.write().await;
        cache
            .deref_mut()
            .set("storm_glass_keys", &keys.values().collect::<Vec<_>>())
            .map_err(|_| ForecastError::CacheError("UNABLE_TO_UPDATE_KEYS".to_string()))
            .await
    }

    async fn keys_from_cache(&self) -> Result<HashMap<String, StormGlassKey>, ForecastError> {
        let cache = self.cache.read().await;
        let maybe_keys = cache
            .get::<Vec<StormGlassKey>>("storm_glass_keys")
            .await
            .map_err(|e| {
                println!("{:#?}", e);
                ForecastError::CacheError("UNABLE_TO_FETCH_KEYS_FROM_CACHE".to_owned())
            })?;
        maybe_keys
            .map(|keys| {
                keys.into_iter()
                    .map(|key: StormGlassKey| (key.key.clone(), key))
                    .collect()
            })
            .ok_or(ForecastError::Uncontrolled("NO_KEYS_AVAILABLE".to_string()))
    }
}

#[async_trait]
impl<C> ForecastRepository for StormGlassRepository<C>
where
    C: Cache + Send + Sync,
{
    type WeatherForecast = StormGlassWeatherForecast;
    type TideForecast = StormGlassTideForecast;
    async fn weather_forecast(
        &self,
        lat: f64,
        lng: f64,
    ) -> Result<Self::WeatherForecast, ForecastError> {
        let url = self.weather_forecast_url(lat, lng);
        let mut keys = self.keys_from_cache().await?;
        let key = keys.values().find(|key| key.is_valid()).unwrap().clone();
        let res = reqwest::Client::new()
            .get(url)
            .header(header::AUTHORIZATION, &key.key)
            .send()
            .map_err(|err| ForecastError::Uncontrolled(err.to_string()))
            .await?;
        match res.status() {
            StatusCode::PAYMENT_REQUIRED => Err(ForecastError::StormGlassApiKeyMaxRequestReached(
                "".to_string(),
            )),
            StatusCode::OK => {
                self.update_keys(&key.key, &mut keys).await?;
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
