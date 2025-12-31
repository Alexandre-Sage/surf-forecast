use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WavesSources {
    pub(crate) height: f64,
    pub(crate) direction: f64,
    pub(crate) period: f64,
}

impl WavesSources {
    pub(crate) fn new(height: f64, direction: f64, period: f64) -> Self {
        Self {
            height,
            direction,
            period,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WavesInput {
    pub(crate) primary_swell: WavesSources,
    pub(crate) secondary_swell: WavesSources,
    pub(crate) wind_waves: WavesSources,
}

impl WavesInput {
    pub(crate) fn new(
        primary_swell: WavesSources,
        secondary_swell: WavesSources,
        wind_waves: WavesSources,
    ) -> Self {
        Self {
            primary_swell,
            secondary_swell,
            wind_waves,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WavesHeightResult {
    rss: f64,
    rss_directional: f64,
}

impl WavesHeightResult {
    pub(crate) fn new(rss: f64, rss_directional: f64) -> Self {
        Self {
            rss,
            rss_directional,
        }
    }

    pub(crate) fn rss(&self) -> f64 {
        self.rss
    }

    pub(crate) fn rss_directional(&self) -> f64 {
        self.rss_directional
    }
}
