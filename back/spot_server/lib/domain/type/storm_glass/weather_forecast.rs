use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub cost: Number,
    pub daily_quota: Number,
    pub end: String,
    pub lat: Number,
    pub lng: Number,
    pub params: Vec<String>,
    pub request_count: Number,
    pub start: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastSources {
    pub dwd: Option<Number>,
    pub icon: Option<Number>,
    pub meteo: Option<Number>,
    pub noaa: Option<Number>,
    pub sg: Option<Number>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StormGlassHourWeatherForecast {
    pub time: chrono::DateTime<Utc>,
    pub air_temperature: ForecastSources,
    pub cloud_cover: ForecastSources,
    pub precipitation: ForecastSources,
    pub pressure: ForecastSources,
    pub secondary_swell_direction: ForecastSources,
    pub secondary_swell_height: ForecastSources,
    pub secondary_swell_period: ForecastSources,
    pub swell_direction: ForecastSources,
    pub swell_height: ForecastSources,
    pub swell_period: ForecastSources,
    pub visibility: ForecastSources,
    pub water_temperature: ForecastSources,
    pub wave_direction: ForecastSources,
    pub wave_height: ForecastSources,
    pub wave_period: ForecastSources,
    pub wind_direction: ForecastSources,
    pub wind_speed: ForecastSources,
    pub wind_wave_direction: ForecastSources,
    pub wind_wave_height: ForecastSources,
    pub wind_wave_period: ForecastSources,
    pub sea_level: Option<ForecastSources>,
    pub current_direction: Option<ForecastSources>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StormGlassWeatherForecast {
    pub hours: Vec<StormGlassHourWeatherForecast>,
    pub meta: Meta,
}
