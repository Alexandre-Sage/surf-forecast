use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    domain::entities::{coordinates::Coordinate, forecast::storm_glass_waves::StormGlassWavesData},
    infrastructure::http::{
        response::{ApiResponse, HandlerResponse},
        state::AppState,
    },
};

#[axum::debug_handler]
pub(crate) async fn storm_glass_forecast_handler(
    State(app_state): State<Arc<AppState>>,
    Path((lat, lng)): Path<(f32, f32)>,
) -> HandlerResponse<Vec<StormGlassWavesData>> {
    let coordinates = Coordinate::new(lat, lng);
    log::debug!("{:#?}", coordinates);
    let result = app_state.storm_glass_forecast.forecast(coordinates).await?;
    Ok(ApiResponse::success(StatusCode::OK, result))
}
