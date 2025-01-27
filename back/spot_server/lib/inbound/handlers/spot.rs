use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use internal::{api::api::ApiSuccess, error::api::ApiError};
use uuid::Uuid;

use crate::{
    domain::{port::spot_repository::SpotRepository, r#type::spot::SpotDto},
    inbound::api::ApiState,
};

pub async fn get_all_spots<U: SpotRepository>(
    State(state): State<Arc<ApiState<U>>>,
) -> Result<ApiSuccess<Vec<SpotDto>>, ApiError> {
    let spots = state.spot_service.get_all().await?;
    Ok(ApiSuccess::new(StatusCode::OK, spots))
}

pub async fn get_spot<U: SpotRepository>(
    State(state): State<Arc<ApiState<U>>>,
    Path(id): Path<Uuid>,
) -> Result<ApiSuccess<SpotDto>, ApiError> {
    let spot = state.spot_service.get_by_id(id).await?;
    Ok(ApiSuccess::new(StatusCode::OK, spot))
}
