use std::usize;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


// COMMENTED FOR LATER FEATURES ON ALL THE FILE
// #[derive(Debug, strum::EnumIter)]
// pub(crate) enum StormGlassSwellSources {
//     NationalOceanicAndAtmosphericAdministration,
//     StormGlass,
//     DeutscherWetterdienst,
//     MeteoFrance,
//     NorwegianMeteorologicalInstitute,
//     EuropeanCentreForMediumRangeWeatherForecasts,
//     DanishMeteorologicalInstitute,
//     FinnishMeteorologicalInstitute,
// }
//
// #[derive(Debug)]
// pub(crate) enum StormGlassWavesDataProps {
//     SecondarySwell,
//     Swell,
//     WindWave,
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct StormGlassWavesData {
    #[serde(rename = "secondarySwellDirection")]
    pub secondary_swell_direction: WavesSourceValues,
    #[serde(rename = "secondarySwellHeight")]
    pub secondary_swell_height: WavesSourceValues,
    #[serde(rename = "secondarySwellPeriod")]
    pub secondary_swell_period: WavesSourceValues,
    #[serde(rename = "swellDirection")]
    pub swell_direction: WavesSourceValues,
    #[serde(rename = "swellHeight")]
    pub swell_height: WavesSourceValues,
    #[serde(rename = "swellPeriod")]
    pub swell_period: WavesSourceValues,
    pub time: DateTime<Utc>,
    #[serde(rename = "waveDirection")]
    pub wave_direction: WavesSourceValues,
    #[serde(rename = "waveHeight")]
    pub wave_height: WavesSourceValues,
    #[serde(rename = "wavePeriod")]
    pub wave_period: WavesSourceValues,
    #[serde(rename = "windWaveDirection")]
    pub wind_wave_direction: WavesSourceValues,
    #[serde(rename = "windWaveHeight")]
    pub wind_wave_height: WavesSourceValues,
    #[serde(rename = "windWavePeriod")]
    pub wind_wave_period: WavesSourceValues,
}

// impl StormGlassWavesData {
//     pub(crate) fn get_source_values(
//         &self,
//         forecat: &StormGlassWavesDataProps,
//         source: &StormGlassSwellSources,
//     ) -> (Option<f64>, Option<f64>, Option<f64>) {
//         match forecat {
//             StormGlassWavesDataProps::Swell => (
//                 self.swell_height.get_source(source),
//                 self.swell_period.get_source(source),
//                 self.swell_direction.get_source(source),
//             ),
//             StormGlassWavesDataProps::SecondarySwell => (
//                 self.secondary_swell_height.get_source(source),
//                 self.secondary_swell_period.get_source(source),
//                 self.secondary_swell_direction.get_source(source),
//             ),
//             StormGlassWavesDataProps::WindWave => (
//                 self.wind_wave_height.get_source(source),
//                 self.wind_wave_period.get_source(source),
//                 self.wind_wave_direction.get_source(source),
//             ),
//         }
//     }
// }

// impl WavesInput {
//     fn proccess(value: (Option<f64>, Option<f64>, Option<f64>)) {}
//     fn from_x(value: StormGlassWavesData, key: StormGlassSwellSources) -> Option<Self> {
//         let primary_swell = value.get_source_values(&StormGlassWavesDataProps::Swell, &key);
//         let secondary_swell =
//             value.get_source_values(&StormGlassWavesDataProps::SecondarySwell, &key);
//         let wind_waves_swell = value.get_source_values(&StormGlassWavesDataProps::WindWave, &key);
//
//         todo!()
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct WavesSourceValues {
    #[serde(rename = "nationalOceanicAndAtmosphericAdministration", alias = "noaa")]
    pub national_oceanic_and_atmospheric_administration: Option<f64>,
    #[serde(rename = "stormGlass", alias = "sg")]
    pub storm_glass: Option<f64>,
    #[serde(rename = "deutscherWetterdienst", alias = "dwd")]
    pub deutscher_wetterdienst: Option<f64>,
    #[serde(rename = "meteoFrance", alias = "meteo")]
    pub meteo_france: Option<f64>,
    #[serde(rename = "norwegianMeteorologicalInstitute", alias = "metno")]
    pub norwegian_meteorological_institute: Option<f64>,
    #[serde(
        rename = "europeanCentreForMediumRangeWeatherForecasts",
        alias = "ecmwf"
    )]
    pub european_centre_for_medium_range_weather_forecasts: Option<f64>,
    #[serde(rename = "danishMeteorologicalInstitute", alias = "fcoo")]
    pub danish_meteorological_institute: Option<f64>,
    #[serde(rename = "finnishMeteorologicalInstitute", alias = "fmi")]
    pub finnish_meteorological_institute: Option<f64>,
    pub average: Option<f64>,
}

// impl WavesSourceValues {
//     fn get_source(&self, source: &StormGlassSwellSources) -> Option<f64> {
//         match source {
//             StormGlassSwellSources::StormGlass => self.storm_glass,
//             StormGlassSwellSources::MeteoFrance => self.meteo_france,
//             StormGlassSwellSources::DeutscherWetterdienst => self.deutscher_wetterdienst,
//             StormGlassSwellSources::DanishMeteorologicalInstitute => {
//                 self.danish_meteorological_institute
//             }
//             StormGlassSwellSources::FinnishMeteorologicalInstitute => {
//                 self.finnish_meteorological_institute
//             }
//             StormGlassSwellSources::NorwegianMeteorologicalInstitute => {
//                 self.norwegian_meteorological_institute
//             }
//             StormGlassSwellSources::NationalOceanicAndAtmosphericAdministration => {
//                 self.national_oceanic_and_atmospheric_administration
//             }
//             StormGlassSwellSources::EuropeanCentreForMediumRangeWeatherForecasts => {
//                 self.european_centre_for_medium_range_weather_forecasts
//             }
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub(crate) struct Waves {
//     pub national_oceanic_and_atmospheric_administration: Option<WavesInput>,
//     pub storm_glass: Option<WavesInput>,
//     pub deutscher_wetterdienst: Option<WavesInput>,
//     pub meteo_france: Option<WavesInput>,
//     pub norwegian_meteorological_institute: Option<WavesInput>,
//     pub european_centre_for_medium_range_weather_forecasts: Option<WavesInput>,
//     pub danish_meteorological_institute: Option<WavesInput>,
//     pub finnish_meteorological_institute: Option<WavesInput>,
// }
