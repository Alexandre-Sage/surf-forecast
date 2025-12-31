use serde::Deserialize;

use crate::domain::entities::forecast::storm_glass_waves::StormGlassWavesData;

#[derive(Debug, Deserialize)]
pub(super) struct StormGlassResponse {
    pub(super) hours: Vec<StormGlassWavesData>,
}
