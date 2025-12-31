use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use reqwest::{header::HeaderValue, StatusCode};
use tokio::sync::Mutex;
use url::{ParseError, Url};

use crate::domain::{
    entities::{coordinates::Coordinate, forecast::storm_glass_waves::StormGlassWavesData},
    port::forecast_provider::{ForecastError, ForecastResult, WavesForecastProvider},
    services::api_keys_service::ApiKeysService,
};

const WAVES_PARAMS: &[&str] = &[
    "swellDirection",
    "swellHeight",
    "swellPeriod",
    "secondarySwellDirection",
    "secondarySwellHeight",
    "secondarySwellPeriod",
    "waveDirection",
    "waveHeight",
    "wavePeriod",
    "windWaveDirection",
    "windWaveHeight",
    "windWavePeriod",
];

pub(crate) struct StormGlassProvider {
    host: String,
    version: String,
    client: reqwest::Client,
    api_keys_service: Arc<Mutex<ApiKeysService>>,
}

impl StormGlassProvider {
    pub fn new(key_manager: Arc<Mutex<ApiKeysService>>) -> Self {
        // TMP hardcoded for test
        Self {
            host: "https://api.stormglass.io".to_owned(),
            version: "v2".to_owned(),
            client: reqwest::Client::new(),
            api_keys_service: key_manager,
        }
    }

    fn base_url(&self) -> Result<Url, ParseError> {
        let path = format!("{}/weather/point", self.version);
        let mut url = Url::from_str(&self.host)?;
        url.set_path(&path);
        Ok(url)
    }

    fn url_coord(&self, coord: &Coordinate) -> Result<Url, ParseError> {
        self.base_url().map(|mut url| {
            let query = format!("lat={}&lng={}", coord.latitude(), coord.longitude());
            url.set_query(Some(&query));
            url
        })
    }

    fn waves_forecast_url(&self, coord: &Coordinate) -> Result<Url, ParseError> {
        self.url_coord(coord).map(|mut url| {
            let params = WAVES_PARAMS.join(",");
            let query = match url.query() {
                Some(query) => format!("{query}&params={params}"),
                None => format!("params={params}"),
            };
            url.set_query(Some(&query));
            url
        })
    }
}

#[async_trait]
impl WavesForecastProvider for StormGlassProvider {
    type Return = Vec<StormGlassWavesData>;

    async fn waves_forecast(&self, coordinates: &Coordinate) -> ForecastResult<Self::Return> {
        let mut api_keys_service = self.api_keys_service.lock().await;

        if api_keys_service.need_rotation() {
            api_keys_service.rotate()?;
        }

        let url = self.waves_forecast_url(coordinates)?;
        let request_builder = self.client.get(url).header(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(api_keys_service.current())
                .map_err(|err| ForecastError::Unknown(err.to_string()))?,
        );

        log::info!("Fetching storm glass API");

        let result = request_builder.send().await?;

        log::debug!("{}", result.status());

        if result.status() != StatusCode::OK {
            return match result.status() {
                StatusCode::FORBIDDEN => Err(ForecastError::Forbidden(result.text().await?)),
                StatusCode::UNPROCESSABLE_ENTITY => {
                    Err(ForecastError::UnprocessableEntity(result.text().await?))
                }
                StatusCode::PAYMENT_REQUIRED => {
                    Err(ForecastError::PaymentRequired(result.text().await?))
                }
                _ => Err(ForecastError::Unknown(result.text().await?)),
            };
        }

        let json = result.json::<super::dtos::StormGlassResponse>().await?;

        api_keys_service.increment_usage().await?;

        Ok(json.hours)
    }
}
