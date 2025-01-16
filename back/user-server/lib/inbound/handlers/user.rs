use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use internal::error::api::ApiError;

use crate::{
    domain::{port::user_repository::UserRepository, r#type::user::UserPayload},
    inbound::api::ApiState,
};

pub async fn create_user<U: UserRepository>(
    State(state): State<Arc<ApiState<U>>>,
    Json(payload): Json<UserPayload>,
) -> Result<StatusCode, ApiError> {
    state.user_service.insert(payload).await?;
    Ok(StatusCode::CREATED)
}
