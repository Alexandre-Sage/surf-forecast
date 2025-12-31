use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct WindData {
    pub gust: WindsSourceValues,
    pub time: String,
    #[serde(rename = "windDirection")]
    pub wind_direction: WindsSourceValues,
    #[serde(rename = "windDirection1000hpa")]
    pub wind_direction_1000hpa: WindsSourceValues,
    #[serde(rename = "windDirection100m")]
    pub wind_direction_100m: WindsSourceValues,
    #[serde(rename = "windDirection200hpa")]
    pub wind_direction_200hpa: WindsSourceValues,
    #[serde(rename = "windDirection20m")]
    pub wind_direction_20m: WindsSourceValues,
    #[serde(rename = "windDirection30m")]
    pub wind_direction_30m: WindsSourceValues,
    #[serde(rename = "windDirection40m")]
    pub wind_direction_40m: WindsSourceValues,
    #[serde(rename = "windDirection500hpa")]
    pub wind_direction_500hpa: WindsSourceValues,
    #[serde(rename = "windDirection50m")]
    pub wind_direction_50m: WindsSourceValues,
    #[serde(rename = "windDirection800hpa")]
    pub wind_direction_800hpa: WindsSourceValues,
    #[serde(rename = "windDirection80m")]
    pub wind_direction_80m: WindsSourceValues,
    #[serde(rename = "windSpeed")]
    pub wind_speed: WindsSourceValues,
    #[serde(rename = "windSpeed1000hpa")]
    pub wind_speed_1000hpa: WindsSourceValues,
    #[serde(rename = "windSpeed100m")]
    pub wind_speed_100m: WindsSourceValues,
    #[serde(rename = "windSpeed200hpa")]
    pub wind_speed_200hpa: WindsSourceValues,
    #[serde(rename = "windSpeed20m")]
    pub wind_speed_20m: WindsSourceValues,
    #[serde(rename = "windSpeed30m")]
    pub wind_speed_30m: WindsSourceValues,
    #[serde(rename = "windSpeed40m")]
    pub wind_speed_40m: WindsSourceValues,
    #[serde(rename = "windSpeed500hpa")]
    pub wind_speed_500hpa: WindsSourceValues,
    #[serde(rename = "windSpeed50m")]
    pub wind_speed_50m: WindsSourceValues,
    #[serde(rename = "windSpeed800hpa")]
    pub wind_speed_800hpa: WindsSourceValues,
    #[serde(rename = "windSpeed80m")]
    pub wind_speed_80m: WindsSourceValues,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct WindsSourceValues {
    #[serde(rename = "nationalOceanicAndAtmosphericAdministration", alias = "noaa")]
    pub national_oceanic_and_atmospheric_administration: Option<f64>,
    #[serde(rename = "stormGlass", alias = "sg")]
    pub storm_glass: Option<f64>,
    #[serde(rename = "deutscherWetterdienst", alias = "dwd")]
    pub deutscher_wetterdienst: Option<f64>,
    #[serde(rename = "norwegianMeteorologicalInstitute", alias = "metno")]
    pub norwegian_meteorological_institute: Option<f64>,
    #[serde(
        rename = "europeanCentreForMediumRangeWeatherForecasts",
        alias = "ecmwf"
    )]
    pub european_centre_for_medium_range_weather_forecasts: Option<f64>,
    #[serde(
        rename = "europeanCentreForMediumRangeWeatherForecastsAifs",
        alias = "ecmwf:aifs"
    )]
    pub european_centre_for_medium_range_weather_forecasts_aifs: Option<f64>,
    #[serde(rename = "swedishMeteorologicalAndHydrologicalInstitute", alias = "smhi")]
    pub swedish_meteorological_and_hydrological_institute: Option<f64>,
}
