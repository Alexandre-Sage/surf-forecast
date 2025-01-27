use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Number;
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TideLevel {
    Low,
    High,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TideHourForecast {
    pub height: Number,
    pub time: DateTime<Utc>,
    pub r#type: TideLevel,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TideStationMeta {
    distance: Number,
    lat: Number,
    lng: Number,
    name: String,
    source: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub cost: Number,
    pub daily_quota: Number,
    pub datum: String,
    pub end: String,
    pub lat: Number,
    pub lng: Number,
    pub request_count: Number,
    pub start: String,
    pub station: TideStationMeta,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StormGlassTideForecast {
    pub data: Vec<TideHourForecast>,
    pub meta: Meta,
}
