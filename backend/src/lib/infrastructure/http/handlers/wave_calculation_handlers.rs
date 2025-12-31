use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    domain::entities::forecast::waves_calculation::{WavesHeightResult, WavesInput},
    infrastructure::http::{
        response::{ApiResponse, HandlerResponse},
        state::AppState,
    },
};

#[axum::debug_handler]
pub(crate) async fn wave_calculation_handler(
    State(app_state): State<Arc<AppState>>,
    Json(input): Json<WavesInput>,
) -> HandlerResponse<WavesHeightResult> {
    let result = app_state.wave_calculation.calculate(input);
    Ok(ApiResponse::success(StatusCode::OK, result))
}
